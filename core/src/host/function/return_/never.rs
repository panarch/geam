#[cfg(test)]
use super::HostFunctionImplementation;
use super::{
    HostCallReturn, HostReturn, HostValueFunction, OwnedHostCallback,
    OwnedHostFunctionImplementation,
};
use crate::host::{
    HostCallArguments, HostCallError, HostCallRuntime, HostFailure, HostProfile, HostTypeDescriptor,
};
use crate::runtime::execution::Continuation;
use std::convert::Infallible;
use std::sync::Arc;

pub(crate) struct HostNeverFunction<Profile: HostProfile> {
    implementation: HostNeverFunctionKind<Profile>,
}

enum HostNeverFunctionKind<Profile: HostProfile> {
    Scalar(OwnedHostCallback<Profile, Infallible>),
    Scoped(Arc<HostScopedNeverCallback<Profile>>),
    Continuing(Arc<HostContinuingNeverCallback<Profile>>),
    Uninhabited(HostValueFunction<Profile>),
}

type HostScopedNeverCallback<Profile> =
    dyn Fn(&mut dyn HostCallRuntime<Profile>) -> Result<Infallible, HostCallError> + Send + Sync;

type HostContinuingNeverCallback<Profile> = dyn Fn(&mut dyn HostCallRuntime<Profile>) -> Result<Continuation<Infallible>, HostCallError>
    + Send
    + Sync;

impl<Profile: HostProfile> Clone for HostNeverFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: match &self.implementation {
                HostNeverFunctionKind::Scalar(function) => {
                    HostNeverFunctionKind::Scalar(function.clone())
                }
                HostNeverFunctionKind::Scoped(function) => {
                    HostNeverFunctionKind::Scoped(Arc::clone(function))
                }
                HostNeverFunctionKind::Continuing(function) => {
                    HostNeverFunctionKind::Continuing(Arc::clone(function))
                }
                HostNeverFunctionKind::Uninhabited(function) => {
                    HostNeverFunctionKind::Uninhabited(function.clone())
                }
            },
        }
    }
}

impl<Profile: HostProfile> HostNeverFunction<Profile> {
    pub(crate) fn start(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<Continuation<Infallible>, HostCallError> {
        match &self.implementation {
            HostNeverFunctionKind::Scalar(function) => {
                let (state, arguments) = runtime.scalar_context();
                function
                    .call(state, arguments)
                    .map(|never| match never {})
                    .map_err(HostCallError::from)
            }
            HostNeverFunctionKind::Scoped(function) => {
                function(runtime).map(|never| match never {})
            }
            HostNeverFunctionKind::Continuing(function) => function(runtime),
            HostNeverFunctionKind::Uninhabited(function) => match function.start(runtime)? {
                HostCallReturn::Immediate(_) => Err(HostFailure::uninhabited_return().into()),
                HostCallReturn::Continuing(continuation) => {
                    let execution = runtime.execution();
                    let codec = runtime.codec_scope();
                    let origin = runtime.origin();
                    Ok(Continuation::new(async move {
                        match continuation.complete().await? {
                            Err(error) => Ok(Err(error)),
                            Ok(value) => {
                                drop(value);
                                execution
                                    .fail_native(
                                        HostFailure::uninhabited_return().into(),
                                        codec,
                                        origin,
                                    )
                                    .await
                            }
                        }
                    }))
                }
            },
        }
    }

    pub(super) fn continuing(
        function: impl Fn(
            &mut dyn HostCallRuntime<Profile>,
        ) -> Result<Continuation<Infallible>, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            implementation: HostNeverFunctionKind::Continuing(Arc::new(function)),
        }
    }

    pub(super) fn scoped(
        function: impl Fn(&mut dyn HostCallRuntime<Profile>) -> Result<Infallible, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            implementation: HostNeverFunctionKind::Scoped(Arc::new(function)),
        }
    }

    pub(super) fn owned(function: OwnedHostCallback<Profile, Infallible>) -> Self {
        Self {
            implementation: HostNeverFunctionKind::Scalar(function),
        }
    }
}

impl<Profile: HostProfile> From<HostValueFunction<Profile>> for HostNeverFunction<Profile> {
    fn from(function: HostValueFunction<Profile>) -> Self {
        Self {
            implementation: HostNeverFunctionKind::Uninhabited(function),
        }
    }
}

impl HostReturn for Infallible {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Parameter(0)
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::Never(OwnedHostCallback::new(function))
    }
}

#[cfg(test)]
pub(crate) fn expect_never_implementation<Profile: HostProfile>(
    implementation: &HostFunctionImplementation<Profile>,
) -> &HostNeverFunction<Profile> {
    let HostFunctionImplementation::Never(implementation) = implementation else {
        panic!("Infallible return should create a Never implementation");
    };
    implementation
}

#[cfg(test)]
mod tests {
    use super::{HostNeverFunction, HostReturn, OwnedHostCallback, expect_never_implementation};
    use crate::execution_fixture::TestHost;
    use crate::host::function::argument::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostCallArguments, HostFailure, HostFunctionImplementation, HostScopedValue,
        HostTypeDescriptor, expect_value_implementation,
    };
    use crate::runtime::execution::Continuation;
    use crate::runtime::run_main;
    use crate::runtime::work::Cancelled;
    use crate::{ExecutionPlan, compile_typed_module, plan_module};
    use std::convert::Infallible;
    use std::future::ready;

    #[test]
    fn cloned_never_bodies_keep_their_original_state_and_failure() {
        let originals = [
            HostNeverFunction::<TestHostProfile>::owned(OwnedHostCallback::new(
                |state: &mut TestRunState, _| {
                    state.counter += 1;
                    Err(HostFailure::new("stopped"))
                },
            )),
            HostNeverFunction::<TestHostProfile>::scoped(|runtime| {
                let (state, _) = runtime.scalar_context();
                state.counter += 1;
                Err(HostFailure::new("stopped").into())
            }),
            HostNeverFunction::<TestHostProfile>::continuing(|runtime| {
                let (state, _) = runtime.scalar_context();
                state.counter += 1;
                Err(HostFailure::new("stopped").into())
            }),
        ];
        for original in originals {
            let alias = original.clone();
            drop(original);
            let mut state = TestRunState::default();
            let mut runtime =
                TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
            assert_eq!(
                alias.start(&mut runtime).err().unwrap().to_string(),
                "stopped"
            );
            assert_eq!(state.counter, 1);
        }
    }

    #[test]
    fn adapting_a_value_body_preserves_errors_and_rejects_completed_values() {
        for succeeds in [false, true] {
            let implementation =
                <() as HostReturn>::implementation::<TestHostProfile>(move |state, _| {
                    state.counter += 1;
                    if succeeds {
                        Ok(())
                    } else {
                        Err(HostFailure::new("original failure"))
                    }
                })
                .into_immediate();
            let adapted =
                HostNeverFunction::from(expect_value_implementation(&implementation).clone());
            let alias = adapted.clone();
            drop(adapted);
            drop(implementation);
            let mut state = TestRunState::default();
            let mut runtime =
                TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
            assert_eq!(
                alias.start(&mut runtime).err().unwrap().to_string(),
                if succeeds {
                    "native call completed with a value for an uninhabited return type"
                } else {
                    "original failure"
                }
            );
            assert_eq!(runtime.completed().is_some(), succeeds);
            assert_eq!(state.counter, 1);
        }
    }

    #[test]
    fn adapted_continuations_preserve_failures_and_cancel_after_scope_shutdown() {
        let typed = compile_typed_module(
            "main",
            "main.gleam",
            "pub fn main() { panic as \"original\" }",
        )
        .unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let original = run_main(&plan, &mut Vec::new()).unwrap_err();
        #[derive(Clone, Copy)]
        enum Outcome {
            SourceFailure,
            Cancelled,
            Value,
        }
        for outcome in [Outcome::SourceFailure, Outcome::Cancelled, Outcome::Value] {
            let expected = if matches!(outcome, Outcome::SourceFailure) {
                Ok(Err(original.clone()))
            } else {
                Err(Cancelled)
            };
            let original = original.clone();
            let implementation =
                HostFunctionImplementation::<TestHostProfile>::continuing(move |runtime| {
                    let output = match outcome {
                        Outcome::SourceFailure => Ok(Err(original.clone())),
                        Outcome::Cancelled => Err(Cancelled),
                        Outcome::Value => Ok(Ok(runtime.retain_stored(HostScopedValue::Nil))),
                    };
                    Ok(Continuation::new(ready(output)))
                });
            let adapted =
                HostNeverFunction::from(expect_value_implementation(&implementation).clone());
            let mut state = TestRunState::default();
            let mut runtime =
                TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
            let continuation = adapted.start(&mut runtime).unwrap();
            drop(runtime);
            let host = TestHost::default();
            assert_eq!(host.block_on(continuation.complete()), expected);
        }
    }

    #[test]
    fn infallible_return_owns_generic_never_callback_and_family() {
        assert_eq!(
            <Infallible as HostReturn>::descriptor(),
            HostTypeDescriptor::Parameter(0),
        );
        let implementation =
            <Infallible as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("stopped"))
            });
        let implementation = implementation.into_immediate();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(
            expect_never_implementation(&implementation)
                .start(&mut runtime)
                .err()
                .expect("non-returning callback should preserve its failure")
                .to_string(),
            "stopped",
        );
    }

    #[test]
    #[should_panic(expected = "Infallible return should create a Never implementation")]
    fn never_return_shape_guard_is_visible() {
        let callback = |_: &mut TestRunState, _: &dyn HostCallArguments| Ok(true);
        let implementation =
            <bool as HostReturn>::implementation::<TestHostProfile>(callback).into_immediate();
        expect_never_implementation(&implementation);
    }
}
