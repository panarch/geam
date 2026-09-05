use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::host::{HostAbiType, HostCallArguments, HostFailure, HostProfile};

pub(super) type HostUtfCodepointFunction<Profile> = OwnedHostCallback<Profile, char>;

impl HostReturn for char {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::UtfCodepoint(OwnedHostCallback::new(function))
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
    fn utf_codepoint_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<char>();
        let implementation =
            <char as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                Ok(arguments.utf_codepoint(slot))
            });
        let implementation = implementation.into_immediate();
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec!['A'],
            0,
        );

        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(
            <char as HostReturn>::descriptor(),
            HostTypeDescriptor::UtfCodepoint
        );
        assert_eq!(
            expect_value_implementation(&implementation)
                .call(&mut runtime)
                .map(|token| token.family),
            Ok(HostValueFamily::UtfCodepoint),
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::UtfCodepoint('A')),
        );
    }
}
