mod bit_array;
mod bool;
mod float;
mod int;
mod never;
mod nil;
mod string;
mod utf_codepoint;

use crate::host::{
    HostCallArguments, HostCallError, HostCallRuntime, HostProfile, HostScopedValue, HostValueToken,
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
}

pub(super) type HostCallback<Profile, Return> = dyn Fn(
        &mut <Profile as HostProfile>::RunState,
        &dyn HostCallArguments,
    ) -> Result<Return, HostCallError>
    + Send
    + Sync;

type HostScopedCallback<Profile> = dyn Fn(&mut dyn HostCallRuntime<Profile>) -> Result<HostValueToken, HostCallError>
    + Send
    + Sync;

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
            },
        }
    }
}

impl<Profile: HostProfile> HostFunctionImplementation<Profile> {
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

    pub(crate) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<HostValueToken, HostCallError> {
        let value = match &self.kind {
            HostValueFunctionKind::Int(function) => HostScopedValue::Int(function.call(runtime)?),
            HostValueFunctionKind::Float(function) => {
                HostScopedValue::Float(function.call(runtime)?)
            }
            HostValueFunctionKind::String(function) => {
                HostScopedValue::String(function.call(runtime)?)
            }
            HostValueFunctionKind::BitArray(function) => {
                HostScopedValue::BitArray(function.call(runtime)?)
            }
            HostValueFunctionKind::UtfCodepoint(function) => {
                HostScopedValue::UtfCodepoint(function.call(runtime)?)
            }
            HostValueFunctionKind::Bool(function) => HostScopedValue::Bool(function.call(runtime)?),
            HostValueFunctionKind::Nil(function) => {
                function.call(runtime)?;
                HostScopedValue::Nil
            }
            HostValueFunctionKind::Scoped(function) => return function(runtime),
        };
        Ok(runtime.complete(value))
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

pub(super) trait HostReturn: Sized {
    fn descriptor() -> crate::host::HostTypeDescriptor;

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile>;
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
            Err(HostCallError::from(HostFailure::new("bool unavailable")))
        });
        let implementation = expect_value_implementation(&implementation);
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));

        assert_eq!(
            implementation.call(&mut runtime),
            Err(HostCallError::from(HostFailure::new("bool unavailable"))),
        );
    }

    #[test]
    #[should_panic(expected = "host callback should produce a value")]
    fn value_return_dispatch_shape_guard_is_visible() {
        let implementation =
            <Infallible as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostCallError::from(HostFailure::new("stopped")))
            });
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
