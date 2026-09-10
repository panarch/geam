mod bit_array;
mod bool;
mod float;
mod int;
mod never;
mod nil;
mod string;
mod utf_codepoint;

use crate::host::{
    HostCallArguments, HostCallError, HostCallRuntime, HostFailure, HostProfile, HostScopedValue,
    HostValueToken,
};
use std::sync::Arc;

use bit_array::HostBitArrayFunction;
use bool::HostBoolFunction;
use float::HostFloatFunction;
use int::HostIntFunction;
pub(crate) use never::HostNeverFunction;
#[cfg(test)]
pub(crate) use never::expect_never_implementation;
use nil::HostNilFunction;
use string::HostStringFunction;
use utf_codepoint::HostUtfCodepointFunction;

pub(crate) enum HostFunctionImplementation<Profile: HostProfile> {
    Never(HostNeverFunction<Profile>),
    Value(HostValueFunction<Profile>),
}

pub(crate) struct HostValueFunction<Profile: HostProfile> {
    kind: HostValueFunctionKind<Profile>,
}

enum HostValueFunctionKind<Profile: HostProfile> {
    Int(HostIntFunction<Profile>),
    Float(HostFloatFunction<Profile>),
    String(HostStringFunction<Profile>),
    BitArray(HostBitArrayFunction<Profile>),
    UtfCodepoint(HostUtfCodepointFunction<Profile>),
    Bool(HostBoolFunction<Profile>),
    Nil(HostNilFunction<Profile>),
    Scoped(Arc<HostScopedCallback<Profile>>),
    Continuing(Arc<HostContinuingCallback<Profile>>),
}

pub(crate) enum HostCallReturn {
    Immediate(HostValueToken),
    Continuing(crate::runtime::execution::Continuation),
}

type HostContinuingCallback<Profile> = dyn Fn(
        &mut dyn HostCallRuntime<Profile>,
    ) -> Result<crate::runtime::execution::Continuation, HostCallError>
    + Send
    + Sync;

pub(crate) type HostCallback<Profile, Return> = dyn Fn(
        &mut <Profile as HostProfile>::RunState,
        &dyn HostCallArguments,
    ) -> Result<Return, HostFailure>
    + Send
    + Sync;

pub(crate) struct OwnedHostCallback<Profile: HostProfile, Return> {
    implementation: Arc<HostCallback<Profile, Return>>,
}

pub(crate) enum OwnedHostFunctionImplementation<Profile: HostProfile> {
    Never(OwnedHostCallback<Profile, std::convert::Infallible>),
    Int(OwnedHostCallback<Profile, num_bigint::BigInt>),
    Float(OwnedHostCallback<Profile, f64>),
    String(OwnedHostCallback<Profile, ecow::EcoString>),
    BitArray(OwnedHostCallback<Profile, crate::BitArrayValue>),
    UtfCodepoint(OwnedHostCallback<Profile, char>),
    Bool(OwnedHostCallback<Profile, bool>),
    Nil(OwnedHostCallback<Profile, ()>),
}

type HostScopedCallback<Profile> = dyn Fn(&mut dyn HostCallRuntime<Profile>) -> Result<HostValueToken, HostCallError>
    + Send
    + Sync;

pub(super) trait HostReturn: Sized {
    fn descriptor() -> crate::host::HostTypeDescriptor;

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile>;
}

impl<Profile: HostProfile, Return> Clone for OwnedHostCallback<Profile, Return> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile, Return> OwnedHostCallback<Profile, Return> {
    pub(super) fn new(
        implementation: impl Fn(
            &mut Profile::RunState,
            &dyn HostCallArguments,
        ) -> Result<Return, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            implementation: Arc::new(implementation),
        }
    }

    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<Return, HostFailure> {
        (self.implementation)(state, arguments)
    }

    fn call_runtime(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<Return, HostCallError> {
        let (state, arguments) = runtime.scalar_context();
        self.call(state, arguments).map_err(HostCallError::from)
    }
}

impl<Profile: HostProfile> OwnedHostFunctionImplementation<Profile> {
    pub(crate) fn into_immediate(self) -> HostFunctionImplementation<Profile> {
        match self {
            Self::Never(function) => {
                HostFunctionImplementation::Never(HostNeverFunction::owned(function))
            }
            Self::Int(function) => {
                HostFunctionImplementation::Value(HostValueFunction::int(function))
            }
            Self::Float(function) => {
                HostFunctionImplementation::Value(HostValueFunction::float(function))
            }
            Self::String(function) => {
                HostFunctionImplementation::Value(HostValueFunction::string(function))
            }
            Self::BitArray(function) => {
                HostFunctionImplementation::Value(HostValueFunction::bit_array(function))
            }
            Self::UtfCodepoint(function) => {
                HostFunctionImplementation::Value(HostValueFunction::utf_codepoint(function))
            }
            Self::Bool(function) => {
                HostFunctionImplementation::Value(HostValueFunction::bool_(function))
            }
            Self::Nil(function) => {
                HostFunctionImplementation::Value(HostValueFunction::nil(function))
            }
        }
    }
}

impl<Profile: HostProfile> Clone for HostValueFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            kind: match &self.kind {
                HostValueFunctionKind::Int(function) => {
                    HostValueFunctionKind::Int(function.clone())
                }
                HostValueFunctionKind::Float(function) => {
                    HostValueFunctionKind::Float(function.clone())
                }
                HostValueFunctionKind::String(function) => {
                    HostValueFunctionKind::String(function.clone())
                }
                HostValueFunctionKind::BitArray(function) => {
                    HostValueFunctionKind::BitArray(function.clone())
                }
                HostValueFunctionKind::UtfCodepoint(function) => {
                    HostValueFunctionKind::UtfCodepoint(function.clone())
                }
                HostValueFunctionKind::Bool(function) => {
                    HostValueFunctionKind::Bool(function.clone())
                }
                HostValueFunctionKind::Nil(function) => {
                    HostValueFunctionKind::Nil(function.clone())
                }
                HostValueFunctionKind::Scoped(function) => {
                    HostValueFunctionKind::Scoped(Arc::clone(function))
                }
                HostValueFunctionKind::Continuing(function) => {
                    HostValueFunctionKind::Continuing(Arc::clone(function))
                }
            },
        }
    }
}

impl<Profile: HostProfile> HostFunctionImplementation<Profile> {
    pub(super) fn continuing(
        function: impl Fn(
            &mut dyn HostCallRuntime<Profile>,
        ) -> Result<crate::runtime::execution::Continuation, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self::Value(HostValueFunction {
            kind: HostValueFunctionKind::Continuing(Arc::new(function)),
        })
    }

    pub(super) fn scoped(
        function: impl Fn(&mut dyn HostCallRuntime<Profile>) -> Result<HostValueToken, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self::Value(HostValueFunction {
            kind: HostValueFunctionKind::Scoped(Arc::new(function)),
        })
    }

    pub(super) fn scoped_never(
        function: impl Fn(
            &mut dyn HostCallRuntime<Profile>,
        ) -> Result<std::convert::Infallible, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self::Never(HostNeverFunction::scoped(function))
    }
}

impl<Profile: HostProfile> HostValueFunction<Profile> {
    fn int(function: HostIntFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Int(function),
        }
    }

    fn float(function: HostFloatFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Float(function),
        }
    }

    fn string(function: HostStringFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::String(function),
        }
    }

    fn bit_array(function: HostBitArrayFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::BitArray(function),
        }
    }

    fn utf_codepoint(function: HostUtfCodepointFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::UtfCodepoint(function),
        }
    }

    fn bool_(function: HostBoolFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Bool(function),
        }
    }

    fn nil(function: HostNilFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Nil(function),
        }
    }

    pub(crate) fn start(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<HostCallReturn, HostCallError> {
        let value = match &self.kind {
            HostValueFunctionKind::Int(function) => {
                HostScopedValue::Int(function.call_runtime(runtime)?)
            }
            HostValueFunctionKind::Float(function) => {
                HostScopedValue::Float(function.call_runtime(runtime)?)
            }
            HostValueFunctionKind::String(function) => {
                HostScopedValue::String(function.call_runtime(runtime)?)
            }
            HostValueFunctionKind::BitArray(function) => {
                HostScopedValue::BitArray(function.call_runtime(runtime)?)
            }
            HostValueFunctionKind::UtfCodepoint(function) => {
                HostScopedValue::UtfCodepoint(function.call_runtime(runtime)?)
            }
            HostValueFunctionKind::Bool(function) => {
                HostScopedValue::Bool(function.call_runtime(runtime)?)
            }
            HostValueFunctionKind::Nil(function) => {
                function.call_runtime(runtime)?;
                HostScopedValue::Nil
            }
            HostValueFunctionKind::Scoped(function) => {
                return function(runtime).map(HostCallReturn::Immediate);
            }
            HostValueFunctionKind::Continuing(function) => {
                return function(runtime).map(HostCallReturn::Continuing);
            }
        };
        Ok(HostCallReturn::Immediate(runtime.complete(value)))
    }
}

#[cfg(test)]
pub(crate) fn expect_immediate_call<Profile: HostProfile>(
    function: &HostValueFunction<Profile>,
    runtime: &mut dyn HostCallRuntime<Profile>,
) -> Result<HostValueToken, HostCallError> {
    match function.start(runtime)? {
        HostCallReturn::Immediate(value) => Ok(value),
        HostCallReturn::Continuing(_) => {
            panic!("the registered native leaf should complete immediately")
        }
    }
}

#[cfg(test)]
pub(crate) fn expect_value_implementation<Profile: HostProfile>(
    implementation: &HostFunctionImplementation<Profile>,
) -> &HostValueFunction<Profile> {
    let HostFunctionImplementation::Value(implementation) = implementation else {
        panic!("host callback should produce a value");
    };
    implementation
}

#[cfg(test)]
mod tests {
    use super::HostReturn;
    use crate::host::function::argument::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{HostCallError, HostFailure, expect_value_implementation};
    use std::convert::Infallible;

    #[test]
    fn value_return_dispatch_preserves_typed_callback_failure() {
        let implementation = <bool as HostReturn>::implementation::<TestHostProfile>(|_, _| {
            Err(HostFailure::new("bool unavailable"))
        })
        .into_immediate();
        let implementation = expect_value_implementation(&implementation);
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));

        assert_eq!(
            crate::host::expect_immediate_call(implementation, &mut runtime),
            Err(HostCallError::from(HostFailure::new("bool unavailable"))),
        );
    }

    #[test]
    #[should_panic(expected = "the registered native leaf should complete immediately")]
    fn immediate_fixture_rejects_a_resumable_implementation() {
        let implementation =
            super::HostFunctionImplementation::<TestHostProfile>::continuing(|_| {
                Ok(crate::runtime::execution::Continuation::new(
                    std::future::ready(Err(crate::runtime::work::Cancelled)),
                ))
            });
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        let _ = super::expect_immediate_call(
            expect_value_implementation(&implementation),
            &mut runtime,
        );
    }

    #[test]
    #[should_panic(expected = "host callback should produce a value")]
    fn value_return_dispatch_shape_guard_is_visible() {
        let implementation =
            <Infallible as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("stopped"))
            })
            .into_immediate();
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        assert_eq!(
            super::never::expect_never_implementation(&implementation).call(&mut runtime),
            Err(HostCallError::from(HostFailure::new("stopped"))),
        );

        expect_value_implementation(&implementation);
    }
}
