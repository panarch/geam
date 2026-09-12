use super::{HostReturn, OwnedHostCallback, OwnedHostFunctionImplementation};
use crate::BitArrayValue;
use crate::host::{HostAbiType, HostCallArguments, HostFailure, HostProfile};

pub(super) type HostBitArrayFunction<Profile> = OwnedHostCallback<Profile, BitArrayValue>;

impl HostReturn for BitArrayValue {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostFailure>
        + Send
        + Sync
        + 'static,
    ) -> OwnedHostFunctionImplementation<Profile> {
        OwnedHostFunctionImplementation::BitArray(OwnedHostCallback::new(function))
    }
}

#[cfg(test)]
mod tests {
    use super::HostReturn;
    use crate::BitArrayValue;
    use crate::host::function::argument::{CallArguments, HostParameterLayout};
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostScopedValue, HostTypeDescriptor, HostValueFamily, expect_value_implementation,
    };

    #[test]
    fn bit_array_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<BitArrayValue>();
        let implementation = <BitArrayValue as HostReturn>::implementation::<TestHostProfile>(
            move |_, arguments| Ok(arguments.bit_array(slot)),
        );
        let implementation = implementation.into_immediate();
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            vec![BitArrayValue::from_bytes(vec![0xa5])],
            Vec::new(),
            0,
        );

        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        assert_eq!(
            <BitArrayValue as HostReturn>::descriptor(),
            HostTypeDescriptor::BitArray,
        );
        assert_eq!(
            crate::host::expect_immediate_call(
                expect_value_implementation(&implementation),
                &mut runtime
            )
            .map(|token| token.family),
            Ok(HostValueFamily::BitArray),
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::BitArray(BitArrayValue::from_bytes(vec![
                0xa5,
            ]))),
        );
    }
}
