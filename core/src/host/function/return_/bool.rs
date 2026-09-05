use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::host::{HostAbiType, HostCallArguments, HostFailure, HostProfile};

pub(super) type HostBoolFunction<Profile> = OwnedHostCallback<Profile, bool>;

impl HostReturn for bool {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::Bool(OwnedHostCallback::new(function))
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
    fn bool_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<bool>();
        let implementation =
            <bool as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                Ok(!arguments.bool(slot))
            });
        let implementation = implementation.into_immediate();
        let arguments = CallArguments::new(Vec::new(), vec![false]);
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(<bool as HostReturn>::descriptor(), HostTypeDescriptor::Bool);
        assert_eq!(
            expect_value_implementation(&implementation)
                .call(&mut runtime)
                .map(|token| token.family),
            Ok(HostValueFamily::Bool),
        );
        assert_eq!(runtime.completed(), Some(&HostScopedValue::Bool(true)));
    }
}
