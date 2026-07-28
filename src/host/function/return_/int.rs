use super::{HostCallback, HostFunctionImplementation, HostReturn, HostValueFunction};
use crate::host::{HostAbiType, HostCallArguments, HostCallError, HostCallRuntime, HostProfile};
use num_bigint::BigInt;
use std::sync::Arc;

pub(super) struct HostIntFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, BigInt>>,
}

impl<Profile: HostProfile> Clone for HostIntFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostIntFunction<Profile> {
    pub(super) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<BigInt, HostCallError> {
        let (state, arguments) = runtime.scalar_context();
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for BigInt {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunction::int(HostIntFunction {
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
    use num_bigint::BigInt;

    #[test]
    fn int_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<BigInt>();
        let implementation =
            <BigInt as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                Ok(arguments.int(slot) + 42)
            });
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
            expect_value_implementation(&implementation)
                .call(&mut runtime)
                .map(|token| token.family),
            Ok(HostValueFamily::Int),
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Int(BigInt::from(42))),
        );
    }
}
