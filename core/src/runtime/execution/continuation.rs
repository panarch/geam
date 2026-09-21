use super::ExecutionContext;
use crate::host::{
    HostCallRuntime, HostCodecScope, HostExecutionError, HostOwnedCompletion, HostProfile,
    HostProvider, HostScopedValue, HostType, HostTypeSequence,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::host::RuntimeHostCall;
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

    pub(in crate::runtime) async fn complete(self) -> Result<ExecutionResult<Output>, Cancelled> {
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
            output.map_err(|error| {
                crate::runtime::host::host_call_error(plan, origin, codec.function(), error)
            })
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
                    Err(crate::runtime::host::host_call_error(
                        plan,
                        origin,
                        codec.function(),
                        error,
                    ))
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
        HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostTypeListEnd,
    };
    use crate::runtime::SharedExecutionError;
    use crate::runtime::shared::Shared;
    use crate::{ModuleSource, PackageSource};
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
                    None => Ok(crate::HostOwnedCompletion::new(|call, _| {
                        Ok(call.return_value(42.into()))
                    })),
                }
            })
        }))
    }

    #[test]
    fn native_cancellation_and_shared_source_failure_keep_their_original_domains() {
        let source_error =
            crate::runtime::run_src_error("pub fn main() { panic as \"original source failure\" }");
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
                        crate::HostFailure::new("native failure").into(),
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
}
