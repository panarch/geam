use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::host::{HostAbiType, HostCallArguments, HostFailure, HostProfile};

pub(super) type HostNilFunction<Profile> = OwnedHostCallback<Profile, ()>;

impl HostReturn for () {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::Nil(OwnedHostCallback::new(function))
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
    fn nil_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<()>();
        let implementation =
            <() as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                arguments.nil(slot);
                Ok(())
            });
        let implementation = implementation.into_immediate();
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            1,
        );

        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(<() as HostReturn>::descriptor(), HostTypeDescriptor::Nil);
        assert_eq!(
            crate::host::expect_immediate_call(
                expect_value_implementation(&implementation),
                &mut runtime
            )
            .map(|token| token.family),
            Ok(HostValueFamily::Nil),
        );
        assert_eq!(runtime.completed(), Some(&HostScopedValue::Nil));
    }
}
