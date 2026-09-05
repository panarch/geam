use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::host::{HostAbiType, HostCallArguments, HostFailure, HostProfile};

pub(super) type HostFloatFunction<Profile> = OwnedHostCallback<Profile, f64>;

impl HostReturn for f64 {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::Float(OwnedHostCallback::new(function))
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
        let implementation = implementation.into_immediate();
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
