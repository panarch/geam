use super::ExecutionContext;
use crate::host::{
    HostCallRuntime, HostCodecScope, HostExecutionError, HostOwnedCompletion, HostProfile,
    HostProvider, HostScopedValue, HostType, HostTypeSequence,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::host::{RuntimeHostCall, host_call_error};
use crate::runtime::work::Cancelled;
use crate::runtime::{HostCallOrigin, StoredRuntimeValue};
use std::future::Future;
use std::pin::Pin;

pub(crate) struct Continuation<Output = StoredRuntimeValue> {
    operation: Pin<Box<dyn Future<Output = Result<ExecutionResult<Output>, Cancelled>> + Send>>,
}

impl<Output> Continuation<Output> {
    pub(crate) fn new(
        operation: impl Future<Output = Result<ExecutionResult<Output>, Cancelled>> + Send + 'static,
    ) -> Self {
        Self {
            operation: Box::pin(operation),
        }
    }

    pub(crate) async fn complete(self) -> Result<ExecutionResult<Output>, Cancelled> {
        self.operation.await
    }
}

impl<Profile: HostProfile> ExecutionContext<Profile> {
    pub(crate) async fn complete_native<Provider, Output, Constructions>(
        &self,
        completion: Result<
            HostOwnedCompletion<Profile, Provider, Output, Constructions>,
            HostExecutionError,
        >,
        codec: HostCodecScope,
        origin: HostCallOrigin,
        callable_base: usize,
    ) -> Result<ExecutionResult<StoredRuntimeValue>, Cancelled>
    where
        Provider: HostProvider<Profile>,
        Output: HostType,
        Constructions: HostTypeSequence,
    {
        let completion = match completion {
            Ok(completion) => completion,
            Err(error) => return self.fail_native(error, codec, origin).await,
        };
        self.with_runtime(move |plan, state| {
            let mut runtime = RuntimeHostCall::new_codec(plan, state, &codec, origin.clone());
            let output = completion
                .complete(&mut runtime, callable_base)
                .map(|token| runtime.retain_stored(HostScopedValue::Value(token)));
            drop(runtime);
            output.map_err(|error| host_call_error(plan, origin, codec.function(), error))
        })
        .await
    }

    pub(crate) async fn fail_native<Output: Send + 'static>(
        &self,
        error: HostExecutionError,
        codec: HostCodecScope,
        origin: HostCallOrigin,
    ) -> Result<ExecutionResult<Output>, Cancelled> {
        match error {
            HostExecutionError::Cancelled => Err(Cancelled),
            HostExecutionError::Execution(error) => Ok(Err(error.0.read(Clone::clone))),
            HostExecutionError::Host(error) => {
                self.with_runtime(move |plan, _| {
                    Err(host_call_error(plan, origin, codec.function(), error))
                })
                .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::embedding::{CallError, FunctionDeclaration, HostedModuleBuilder};
    use crate::execution_fixture::TestHost;
    use crate::host::{
        HostCall, HostCallContinuation, HostCallError, HostConstructions, HostExecutionError,
        HostFailure, HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule,
        HostProviderSet, HostTypeListEnd,
    };
    use crate::runtime::shared::Shared;
    use crate::runtime::{SharedExecutionError, run_src_error};
    use crate::{ModuleSource, PackageSource, compile_typed_host_program};
    use num_bigint::BigInt;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = Option<HostExecutionError>;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    struct Provider;
    impl HostProvider<Profile> for Provider {
        type State = Option<HostExecutionError>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }

    fn complete<'call>(
        mut call: HostCall<'call, Profile, Provider, BigInt>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
    ) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
        let error = call.state().take();
        Ok(call.resume(constructions, move |_| {
            Box::pin(async move {
                match error {
                    Some(error) => Err(error),
                    None => Ok(HostOwnedCompletion::new(|call, _| {
                        Ok(call.return_value(42.into()))
                    })),
                }
            })
        }))
    }

    #[test]
    fn native_cancellation_and_shared_source_failure_keep_their_original_domains() {
        let source_error = run_src_error("pub fn main() { panic as \"original source failure\" }");
        for (native, expected) in [
            (HostExecutionError::Cancelled, CallError::Cancelled),
            (
                HostExecutionError::Execution(SharedExecutionError(Shared::new(
                    source_error.clone(),
                ))),
                CallError::Execution(source_error),
            ),
        ] {
            let providers = HostProviderSet::from_providers([HostProviderModule::new(
                "application",
                "library",
            )
            .unwrap()
            .with_resumable_function::<Provider, (), BigInt, HostTypeListEnd, _>(
                "complete", complete,
            )
            .unwrap()])
            .unwrap();
            let typed = compile_typed_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
@external(erlang, "native", "complete")
fn complete() -> Int
pub fn run() { let value = complete() echo "completed" value }
"#,
                    )],
                )],
                providers,
            )
            .unwrap();
            let (bindings, run) = HostedModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), BigInt>::new("run"))
                .unwrap();
            let mut module = bindings.seal().unwrap();
            let host = TestHost::default();
            let mut state = Some(native);
            let mut echo = Vec::new();
            let result = host
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(&run, ()).await
                    }),
                )
                .unwrap();
            assert_eq!(result, Err(expected));
            assert!(state.is_none());
            assert!(echo.is_empty());

            for (next, expected, expected_echo) in [
                (None, Ok(BigInt::from(42)), vec!["\"completed\""]),
                (
                    Some(HostExecutionError::Host(
                        HostFailure::new("native failure").into(),
                    )),
                    Err("host function application::library.complete failed: native failure"),
                    vec![],
                ),
            ] {
                state = next;
                echo.clear();
                let result = host
                    .block_on(
                        module.with_execution(&host, &mut state, &mut echo, async |scope| {
                            scope.call(&run, ()).await
                        }),
                    )
                    .unwrap();
                assert_eq!(
                    result.map_err(|error| error.to_string()),
                    expected.map_err(str::to_owned)
                );
                assert!(state.is_none());
                assert_eq!(
                    echo.iter()
                        .map(|output| output.value().inspect().to_string())
                        .collect::<Vec<_>>(),
                    expected_echo
                );
            }
        }
    }

    #[test]
    fn unresolved_native_completion_runs_its_codec_and_preserves_late_errors() {
        use crate::host::HostTypeParameter;
        struct CodecProfile;
        impl HostProfile for CodecProfile {
            type RunState = Vec<&'static str>;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        impl HostProvider<CodecProfile> for CodecProfile {
            type State = Vec<&'static str>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        type Item = HostTypeParameter<0>;
        fn complete<'call>(
            mut call: HostCall<'call, CodecProfile, CodecProfile, Item>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            late: bool,
        ) -> Result<HostCallContinuation<'call, Item>, HostCallError> {
            call.state().push("entered");
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    context
                        .with_state(|state| state.push("resumed"))
                        .await
                        .expect("active test execution admits state access");
                    if !late {
                        return Err(HostFailure::new("async stopped").into());
                    }
                    Ok(HostOwnedCompletion::<
                        CodecProfile,
                        CodecProfile,
                        Item,
                        HostTypeListEnd,
                    >::new(|mut call, _| {
                        call.state().push("codec");
                        Err(HostFailure::new("codec stopped").into())
                    }))
                })
            }))
        }
        let source = r#"
@external(erlang, "native", "complete")
fn complete(late: Bool) -> a
pub fn run(late: Bool) { echo "before" let _ = complete(late) echo "after" 42 }
"#;
        let hosts =
            HostProviderSet::from_providers([HostProviderModule::new("application", "library")
                .unwrap()
                .with_resumable_function::<CodecProfile, (bool,), Item, HostTypeListEnd, _>(
                    "complete", complete,
                )
                .unwrap()])
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("library", "library.gleam", source)],
            )],
            hosts,
        )
        .unwrap();
        let (bindings, run) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(bool,), BigInt>::new("run"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = TestHost::default();
        for late in [false, true] {
            let mut state = Vec::new();
            let mut echo = Vec::new();
            let result = host
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(&run, (late,)).await
                    }),
                )
                .unwrap();
            assert_eq!(
                result.unwrap_err().to_string(),
                if late {
                    "host function application::library.complete failed: codec stopped"
                } else {
                    "host function application::library.complete failed: async stopped"
                }
            );
            assert_eq!(
                state,
                if late {
                    vec!["entered", "resumed", "codec"]
                } else {
                    vec!["entered", "resumed"]
                }
            );
            assert_eq!(
                echo.iter()
                    .map(|echo| echo.value().inspect().to_string())
                    .collect::<Vec<_>>(),
                ["\"before\""]
            );
        }
    }
}
