mod bit_array;
mod bool;
mod float;
mod int;
mod never;
mod nil;
mod string;
mod utf_codepoint;

use crate::host::{
    HostCallArguments, HostCallError, HostCallRuntime, HostCodecScope, HostFailure, HostProfile,
    HostScopedValue, HostValueToken,
};
use crate::plan::execution::host::HostNativeView;
use crate::runtime::StoredRuntimeValue;
use crate::runtime::execution::Continuation;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HostFunctionBinding<Value, Never, Views = ()> {
    Never(Never),
    Value(Value),
    NativeValue(Value, Views),
}

pub(crate) type HostFunctionImplementation<Profile> = HostFunctionBinding<
    HostValueFunction<Profile>,
    HostNeverFunction<Profile>,
    HostNativeViewFactory<Profile>,
>;

pub(crate) struct HostValueFunction<Profile: HostProfile> {
    kind: HostValueFunctionKind<Profile>,
}

#[derive(Clone)]
pub(crate) struct NativeViewBinding {
    pub(crate) codec: HostCodecScope,
    pub(crate) view: HostNativeView,
}

pub(crate) trait NativeViewImplementation<Value> {
    fn native_view(&self, binding: NativeViewBinding) -> Value;
}

impl NativeViewImplementation<()> for () {
    fn native_view(&self, _binding: NativeViewBinding) {}
}

pub(crate) struct HostNativeViewFactory<Profile: HostProfile> {
    callback: Arc<HostNativeViewCallback<Profile>>,
}

impl<Profile: HostProfile> HostNativeViewFactory<Profile> {
    pub(in crate::host) fn new(
        callback: impl Fn(
            &mut dyn HostCallRuntime<Profile>,
            &NativeViewBinding,
        ) -> Result<Continuation, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            callback: Arc::new(callback),
        }
    }
}

type HostNativeViewCallback<Profile> = dyn Fn(&mut dyn HostCallRuntime<Profile>, &NativeViewBinding) -> Result<Continuation, HostCallError>
    + Send
    + Sync;

enum HostValueFunctionKind<Profile: HostProfile> {
    Synchronous(SynchronousValueFunction<Profile>),
    Continuing(Arc<HostContinuingCallback<Profile>>),
}

/// An implementation phase selected by its typed registration owner, never by
/// running a callback or changing its canonical function identity.
pub(crate) struct SynchronousValueFunction<Profile: HostProfile> {
    kind: SynchronousValueFunctionKind<Profile>,
}

enum SynchronousValueFunctionKind<Profile: HostProfile> {
    Int(HostIntFunction<Profile>),
    Float(HostFloatFunction<Profile>),
    String(HostStringFunction<Profile>),
    BitArray(HostBitArrayFunction<Profile>),
    UtfCodepoint(HostUtfCodepointFunction<Profile>),
    Bool(HostBoolFunction<Profile>),
    Nil(HostNilFunction<Profile>),
    Scoped(Arc<HostScopedCallback<Profile>>),
    Retained(Arc<RetainedCallbacks<Profile>>),
}

pub(crate) type HostRetainedCallback =
    dyn Fn(&StoredRuntimeValue) -> Result<StoredRuntimeValue, HostCallError> + Send + Sync;

struct RetainedCallbacks<Profile: HostProfile> {
    scoped: Arc<HostScopedCallback<Profile>>,
    retained: Arc<HostRetainedCallback>,
}

pub(crate) enum HostCallReturn {
    Immediate(HostValueToken),
    Continuing(Continuation),
}

type HostContinuingCallback<Profile> =
    dyn Fn(&mut dyn HostCallRuntime<Profile>) -> Result<Continuation, HostCallError> + Send + Sync;

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
    String(OwnedHostCallback<Profile, crate::StringValue>),
    BitArray(OwnedHostCallback<Profile, crate::BitArrayValue>),
    UtfCodepoint(OwnedHostCallback<Profile, char>),
    Bool(OwnedHostCallback<Profile, bool>),
    Nil(OwnedHostCallback<Profile, ()>),
}

pub(super) type HostScopedCallback<Profile> = dyn Fn(&mut dyn HostCallRuntime<Profile>) -> Result<HostValueToken, HostCallError>
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
                HostValueFunctionKind::Synchronous(function) => {
                    HostValueFunctionKind::Synchronous(function.clone())
                }
                HostValueFunctionKind::Continuing(function) => {
                    HostValueFunctionKind::Continuing(Arc::clone(function))
                }
            },
        }
    }
}

impl<Profile: HostProfile> Clone for SynchronousValueFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            kind: match &self.kind {
                SynchronousValueFunctionKind::Int(function) => {
                    SynchronousValueFunctionKind::Int(function.clone())
                }
                SynchronousValueFunctionKind::Float(function) => {
                    SynchronousValueFunctionKind::Float(function.clone())
                }
                SynchronousValueFunctionKind::String(function) => {
                    SynchronousValueFunctionKind::String(function.clone())
                }
                SynchronousValueFunctionKind::BitArray(function) => {
                    SynchronousValueFunctionKind::BitArray(function.clone())
                }
                SynchronousValueFunctionKind::UtfCodepoint(function) => {
                    SynchronousValueFunctionKind::UtfCodepoint(function.clone())
                }
                SynchronousValueFunctionKind::Bool(function) => {
                    SynchronousValueFunctionKind::Bool(function.clone())
                }
                SynchronousValueFunctionKind::Nil(function) => {
                    SynchronousValueFunctionKind::Nil(function.clone())
                }
                SynchronousValueFunctionKind::Scoped(function) => {
                    SynchronousValueFunctionKind::Scoped(Arc::clone(function))
                }
                SynchronousValueFunctionKind::Retained(function) => {
                    SynchronousValueFunctionKind::Retained(Arc::clone(function))
                }
            },
        }
    }
}

impl<Profile: HostProfile> HostFunctionImplementation<Profile> {
    pub(super) fn continuing_never(
        function: impl Fn(
            &mut dyn HostCallRuntime<Profile>,
        ) -> Result<Continuation<std::convert::Infallible>, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self::Never(HostNeverFunction::continuing(function))
    }

    pub(super) fn continuing(
        function: impl Fn(&mut dyn HostCallRuntime<Profile>) -> Result<Continuation, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self::Value(HostValueFunction::continuing(function))
    }

    pub(super) fn scoped(
        function: impl Fn(&mut dyn HostCallRuntime<Profile>) -> Result<HostValueToken, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self::scoped_callback(Arc::new(function))
    }

    pub(super) fn scoped_callback(scoped: Arc<HostScopedCallback<Profile>>) -> Self {
        Self::Value(HostValueFunction {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::Scoped(scoped),
            }),
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

    pub(super) fn retained(
        scoped: Arc<HostScopedCallback<Profile>>,
        retained: Arc<HostRetainedCallback>,
    ) -> Self {
        Self::Value(HostValueFunction {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::Retained(Arc::new(RetainedCallbacks {
                    scoped,
                    retained,
                })),
            }),
        })
    }
}

impl<Profile: HostProfile> NativeViewImplementation<HostValueFunction<Profile>>
    for HostNativeViewFactory<Profile>
{
    fn native_view(&self, binding: NativeViewBinding) -> HostValueFunction<Profile> {
        let callback = Arc::clone(&self.callback);
        HostValueFunction::continuing(move |runtime| callback(runtime, &binding))
    }
}

impl<Profile: HostProfile> HostValueFunction<Profile> {
    pub(in crate::host) fn scoped(
        function: impl Fn(&mut dyn HostCallRuntime<Profile>) -> Result<HostValueToken, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::Scoped(Arc::new(function)),
            }),
        }
    }

    pub(in crate::host) fn continuing(
        function: impl Fn(&mut dyn HostCallRuntime<Profile>) -> Result<Continuation, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            kind: HostValueFunctionKind::Continuing(Arc::new(function)),
        }
    }

    pub(crate) fn retained(&self) -> Option<Arc<HostRetainedCallback>> {
        self.synchronous()?.retained()
    }

    pub(crate) fn synchronous(&self) -> Option<&SynchronousValueFunction<Profile>> {
        match &self.kind {
            HostValueFunctionKind::Synchronous(function) => Some(function),
            HostValueFunctionKind::Continuing(_) => None,
        }
    }

    fn int(function: HostIntFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::Int(function),
            }),
        }
    }

    fn float(function: HostFloatFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::Float(function),
            }),
        }
    }

    fn string(function: HostStringFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::String(function),
            }),
        }
    }

    fn bit_array(function: HostBitArrayFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::BitArray(function),
            }),
        }
    }

    fn utf_codepoint(function: HostUtfCodepointFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::UtfCodepoint(function),
            }),
        }
    }

    fn bool_(function: HostBoolFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::Bool(function),
            }),
        }
    }

    fn nil(function: HostNilFunction<Profile>) -> Self {
        Self {
            kind: HostValueFunctionKind::Synchronous(SynchronousValueFunction {
                kind: SynchronousValueFunctionKind::Nil(function),
            }),
        }
    }

    pub(crate) fn start(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<HostCallReturn, HostCallError> {
        match &self.kind {
            HostValueFunctionKind::Synchronous(function) => {
                function.start(runtime).map(HostCallReturn::Immediate)
            }
            HostValueFunctionKind::Continuing(function) => {
                function(runtime).map(HostCallReturn::Continuing)
            }
        }
    }
}

impl<Profile: HostProfile> SynchronousValueFunction<Profile> {
    fn retained(&self) -> Option<Arc<HostRetainedCallback>> {
        match &self.kind {
            SynchronousValueFunctionKind::Retained(callbacks) => {
                Some(Arc::clone(&callbacks.retained))
            }
            _ => None,
        }
    }

    pub(crate) fn start(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<HostValueToken, HostCallError> {
        let value = match &self.kind {
            SynchronousValueFunctionKind::Int(function) => {
                HostScopedValue::Int(function.call_runtime(runtime)?)
            }
            SynchronousValueFunctionKind::Float(function) => {
                HostScopedValue::Float(function.call_runtime(runtime)?)
            }
            SynchronousValueFunctionKind::String(function) => {
                HostScopedValue::String(function.call_runtime(runtime)?)
            }
            SynchronousValueFunctionKind::BitArray(function) => {
                HostScopedValue::BitArray(function.call_runtime(runtime)?)
            }
            SynchronousValueFunctionKind::UtfCodepoint(function) => {
                HostScopedValue::UtfCodepoint(function.call_runtime(runtime)?)
            }
            SynchronousValueFunctionKind::Bool(function) => {
                HostScopedValue::Bool(function.call_runtime(runtime)?)
            }
            SynchronousValueFunctionKind::Nil(function) => {
                function.call_runtime(runtime)?;
                HostScopedValue::Nil
            }
            SynchronousValueFunctionKind::Scoped(function) => {
                return function(runtime);
            }
            SynchronousValueFunctionKind::Retained(function) => {
                return (function.scoped)(runtime);
            }
        };
        Ok(runtime.complete(value))
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
    use crate::runtime::execution::Continuation;
    use std::convert::Infallible;

    #[test]
    fn continuing_callbacks_have_no_synchronous_or_retained_entry() {
        let function = super::HostValueFunction::<TestHostProfile>::continuing(|_| {
            Ok(Continuation::new(std::future::ready(Err(
                crate::runtime::work::Cancelled,
            ))))
        });
        assert!(function.synchronous().is_none());
        assert!(function.retained().is_none());
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        function.start(&mut runtime).unwrap();
        assert_eq!(runtime.completed(), None);
    }

    #[test]
    fn retained_registration_clones_both_entries_and_preserves_their_result_and_failure() {
        use crate::host::{HostScopedValue, HostValueFamily};
        use crate::runtime::BorrowedValue;
        use std::sync::Arc;

        let input = super::StoredRuntimeValue::test_int(42.into());
        for fails in [false, true] {
            let implementation = super::HostFunctionImplementation::<TestHostProfile>::retained(
                Arc::new(move |runtime| {
                    if fails {
                        Err(HostCallError::from(HostFailure::new(
                            "retained unavailable",
                        )))
                    } else {
                        Ok(runtime.complete(HostScopedValue::Int(42.into())))
                    }
                }),
                Arc::new(move |input| {
                    if fails {
                        Err(HostCallError::from(HostFailure::new(
                            "retained unavailable",
                        )))
                    } else {
                        Ok(input.clone_retained())
                    }
                }),
            );
            let original = expect_value_implementation(&implementation);
            let cloned = original.clone();
            let original_entry = original.retained().unwrap();
            let cloned_entry = cloned.retained().unwrap();
            assert!(Arc::ptr_eq(&original_entry, &cloned_entry));
            assert_eq!(
                cloned_entry(&input).map(|returned| BorrowedValue::from_stored(&returned)
                    .int()
                    .bigint()
                    .into_owned()),
                if fails {
                    Err(HostCallError::from(HostFailure::new(
                        "retained unavailable",
                    )))
                } else {
                    Ok(42.into())
                }
            );
            let mut state = TestRunState::default();
            let mut runtime =
                TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
            assert_eq!(
                super::expect_immediate_call(&cloned, &mut runtime).map(|token| token.family),
                if fails {
                    Err(HostCallError::from(HostFailure::new(
                        "retained unavailable",
                    )))
                } else {
                    Ok(HostValueFamily::Int)
                }
            );
            let expected = if fails {
                None
            } else {
                Some(HostScopedValue::Int(42.into()))
            };
            assert_eq!(runtime.completed(), expected.as_ref());
        }
    }

    #[test]
    fn value_return_dispatch_preserves_typed_callback_failure() {
        use crate::{BitArrayValue, StringValue};
        use num_bigint::BigInt;

        let implementations = [
            <BigInt as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("value unavailable"))
            }),
            <f64 as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("value unavailable"))
            }),
            <StringValue as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("value unavailable"))
            }),
            <BitArrayValue as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("value unavailable"))
            }),
            <char as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("value unavailable"))
            }),
            <bool as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("value unavailable"))
            }),
            <() as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostFailure::new("value unavailable"))
            }),
        ];
        for implementation in implementations {
            let implementation = implementation.into_immediate();
            let cloned = expect_value_implementation(&implementation).clone();
            let mut state = TestRunState::default();
            let mut runtime =
                TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
            assert_eq!(
                super::expect_immediate_call(&cloned, &mut runtime),
                Err(HostCallError::from(HostFailure::new("value unavailable"))),
            );
            assert_eq!(runtime.completed(), None);
        }
    }

    #[test]
    #[should_panic(expected = "the registered native leaf should complete immediately")]
    fn immediate_fixture_rejects_a_resumable_implementation() {
        let implementation =
            super::HostFunctionImplementation::<TestHostProfile>::continuing(|_| {
                Ok(Continuation::new(std::future::ready(Err(
                    crate::runtime::work::Cancelled,
                ))))
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
            super::never::expect_never_implementation(&implementation)
                .start(&mut runtime)
                .err(),
            Some(HostCallError::from(HostFailure::new("stopped"))),
        );

        expect_value_implementation(&implementation);
    }
}
