use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::host::{HostAbiType, HostCallArguments, HostFailure, HostProfile};
use ecow::EcoString;

pub(super) type HostStringFunction<Profile> = OwnedHostCallback<Profile, EcoString>;

impl HostReturn for EcoString {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::String(OwnedHostCallback::new(function))
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
    use ecow::EcoString;

    #[test]
    fn string_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<EcoString>();
        let implementation =
            <EcoString as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                Ok(format!("{}!", arguments.string(slot)).into())
            });
        let implementation = implementation.into_immediate();
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            vec!["hello".into()],
            Vec::new(),
            Vec::new(),
            0,
        );

        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(
            <EcoString as HostReturn>::descriptor(),
            HostTypeDescriptor::String,
        );
        assert_eq!(
            crate::host::expect_immediate_call(
                expect_value_implementation(&implementation),
                &mut runtime
            )
            .map(|token| token.family),
            Ok(HostValueFamily::String),
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::String("hello!".into())),
        );
    }
}
