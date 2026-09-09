use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::host::{
    HostCallArguments, HostCallError, HostCallRuntime, HostFailure, HostProfile, HostTypeDescriptor,
};
use std::convert::Infallible;
use std::sync::Arc;

pub(crate) struct HostNeverFunction<Profile: HostProfile> {
    implementation: HostNeverFunctionKind<Profile>,
}

enum HostNeverFunctionKind<Profile: HostProfile> {
    Scalar(OwnedHostCallback<Profile, Infallible>),
    Scoped(Arc<HostScopedNeverCallback<Profile>>),
}

type HostScopedNeverCallback<Profile> =
    dyn Fn(&mut dyn HostCallRuntime<Profile>) -> Result<Infallible, HostCallError> + Send + Sync;

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
            },
        }
    }
}

impl<Profile: HostProfile> HostNeverFunction<Profile> {
    pub(crate) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<Infallible, HostCallError> {
        match &self.implementation {
            HostNeverFunctionKind::Scalar(function) => {
                let (state, arguments) = runtime.scalar_context();
                function.call(state, arguments).map_err(HostCallError::from)
            }
            HostNeverFunctionKind::Scoped(function) => function(runtime),
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
    implementation: &super::HostFunctionImplementation<Profile>,
) -> &HostNeverFunction<Profile> {
    let super::HostFunctionImplementation::Never(implementation) = implementation else {
        panic!("Infallible return should create a Never implementation");
    };
    implementation
}

#[cfg(test)]
mod tests {
    use super::{HostReturn, expect_never_implementation};
    use crate::host::function::argument::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{HostCallArguments, HostFailure, HostTypeDescriptor};
    use std::convert::Infallible;

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
                .call(&mut runtime)
                .expect_err("non-returning callback should preserve its failure")
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
