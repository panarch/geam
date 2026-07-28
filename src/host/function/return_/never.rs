use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::{
    HostCallArguments, HostCallError, HostCallRuntime, HostProfile, HostTypeDescriptor,
};
use std::convert::Infallible;
use std::sync::Arc;

pub(crate) struct HostNeverFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, Infallible>>,
}

impl<Profile: HostProfile> Clone for HostNeverFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostNeverFunction<Profile> {
    pub(crate) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<Infallible, HostCallError> {
        let (state, arguments) = runtime.scalar_context();
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for Infallible {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Parameter(0)
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Never(HostNeverFunction {
            implementation: Arc::new(function),
        })
    }
}

#[cfg(test)]
pub(super) fn expect_never_implementation<Profile: HostProfile>(
    implementation: &HostFunctionImplementation<Profile>,
) -> &HostNeverFunction<Profile> {
    let HostFunctionImplementation::Never(implementation) = implementation else {
        panic!("Infallible return should create a Never implementation");
    };
    implementation
}

#[cfg(test)]
mod tests {
    use super::{HostReturn, expect_never_implementation};
    use crate::host::function::argument::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{HostCallArguments, HostCallError, HostFailure, HostTypeDescriptor};
    use std::convert::Infallible;

    #[test]
    fn infallible_return_owns_generic_never_callback_and_family() {
        assert_eq!(
            <Infallible as HostReturn>::descriptor(),
            HostTypeDescriptor::Parameter(0),
        );
        let implementation =
            <Infallible as HostReturn>::implementation::<TestHostProfile>(|_, _| {
                Err(HostCallError::from(HostFailure::new("stopped")))
            });
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
        let implementation = <bool as HostReturn>::implementation::<TestHostProfile>(callback);
        expect_never_implementation(&implementation);
    }
}
