use super::{HostCallback, HostFunctionImplementation, HostReturn, HostValueFunction};
use crate::host::{HostAbiType, HostCallArguments, HostCallError, HostCallRuntime, HostProfile};
use std::sync::Arc;

pub(super) struct HostFloatFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, f64>>,
}

impl<Profile: HostProfile> Clone for HostFloatFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostFloatFunction<Profile> {
    pub(super) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<f64, HostCallError> {
        let (state, arguments) = runtime.scalar_context();
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for f64 {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunction::float(HostFloatFunction {
            implementation: Arc::new(function),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::HostReturn;
    use crate::host::function::argument::{CallArguments, HostParameterLayout};
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostScopedValue, HostTypeDescriptor, HostValueFamily, expect_value_implementation,
    };

    #[test]
    fn float_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<f64>();
        let implementation =
            <f64 as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                Ok(arguments.float(slot) + 0.5)
            });
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            vec![1.0],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            0,
        );

        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(<f64 as HostReturn>::descriptor(), HostTypeDescriptor::Float);
        assert_eq!(
            expect_value_implementation(&implementation)
                .call(&mut runtime)
                .map(|token| token.family),
            Ok(HostValueFamily::Float),
        );
        assert_eq!(runtime.completed(), Some(&HostScopedValue::Float(1.5)));
    }
}
