use super::{HostCallback, HostFunctionImplementation, HostReturn, HostValueFunction};
use crate::host::{HostAbiType, HostCallArguments, HostCallError, HostCallRuntime, HostProfile};
use std::sync::Arc;

pub(super) struct HostBoolFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, bool>>,
}

impl<Profile: HostProfile> Clone for HostBoolFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostBoolFunction<Profile> {
    pub(super) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<bool, HostCallError> {
        let (state, arguments) = runtime.scalar_context();
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for bool {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunction::bool_(HostBoolFunction {
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
    fn bool_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<bool>();
        let implementation =
            <bool as HostReturn>::implementation::<TestHostProfile>(move |_, arguments| {
                Ok(!arguments.bool(slot))
            });
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
