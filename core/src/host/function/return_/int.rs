use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::host::{HostAbiType, HostCallArguments, HostFailure, HostProfile};
use num_bigint::BigInt;

pub(super) type HostIntFunction<Profile> = OwnedHostCallback<Profile, BigInt>;

impl HostReturn for BigInt {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::Int(OwnedHostCallback::new(function))
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
    use num_bigint::BigInt;

    #[test]
    fn int_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<BigInt>();
        let implementation =
            <BigInt as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                Ok(arguments.int(slot) + 42)
            });
        let implementation = implementation.into_immediate();
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(
            &mut state,
            CallArguments::new(vec![BigInt::from(0)], Vec::new()),
        );

        assert_eq!(
            <BigInt as HostReturn>::descriptor(),
            HostTypeDescriptor::Int
        );
        assert_eq!(
            crate::host::expect_immediate_call(
                expect_value_implementation(&implementation),
                &mut runtime
            )
            .map(|token| token.family),
            Ok(HostValueFamily::Int),
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Int(BigInt::from(42))),
        );
    }
}
