use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
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
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<Infallible, HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for Infallible {
    fn type_() -> HostValueType {
        HostValueType::Parameter(crate::plan::TypeParameterId(0))
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
mod tests {
    use super::{HostFunctionImplementation, HostNeverFunction, HostReturn};
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments};
    use crate::host::{HostCallError, HostFailure, StatelessHostProfile};
    use std::convert::Infallible;

    #[test]
    fn infallible_return_owns_generic_never_callback_and_family() {
        assert_eq!(
            <Infallible as HostReturn>::type_(),
            HostValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let implementation =
            <Infallible as HostReturn>::implementation::<StatelessHostProfile>(|(), _| {
                Err(HostCallError::from(HostFailure::new("stopped")))
            });
        let arguments = CallArguments::new(Vec::new(), Vec::new());

        assert_eq!(
            never_implementation(implementation)
                .call(&mut (), &arguments)
                .expect_err("non-returning callback should preserve its failure")
                .to_string(),
            "stopped",
        );
    }

    #[test]
    #[should_panic(expected = "Infallible return should create a Never implementation")]
    fn never_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(true);
        let implementation = <bool as HostReturn>::implementation::<StatelessHostProfile>(callback);
        never_implementation(implementation);
    }

    fn never_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostNeverFunction<StatelessHostProfile> {
        let HostFunctionImplementation::Never(implementation) = implementation else {
            panic!("Infallible return should create a Never implementation");
        };
        implementation
    }
}
