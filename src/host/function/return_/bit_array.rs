use super::{HostCallback, HostFunctionImplementation, HostReturn, HostValueFunction};
use crate::BitArrayValue;
use crate::host::{HostAbiType, HostCallArguments, HostCallError, HostCallRuntime, HostProfile};
use std::sync::Arc;

pub(super) struct HostBitArrayFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, BitArrayValue>>,
}

impl<Profile: HostProfile> Clone for HostBitArrayFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostBitArrayFunction<Profile> {
    pub(super) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<BitArrayValue, HostCallError> {
        let (state, arguments) = runtime.scalar_context();
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for BitArrayValue {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunction::bit_array(HostBitArrayFunction {
            implementation: Arc::new(function),
        }))
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
            expect_value_implementation(&implementation)
                .call(&mut runtime)
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
