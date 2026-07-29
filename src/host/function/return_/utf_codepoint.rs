use super::{HostCallback, HostFunctionImplementation, HostReturn, HostValueFunction};
use crate::host::{HostAbiType, HostCallArguments, HostCallError, HostCallRuntime, HostProfile};
use std::sync::Arc;

pub(super) struct HostUtfCodepointFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, char>>,
}

impl<Profile: HostProfile> Clone for HostUtfCodepointFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostUtfCodepointFunction<Profile> {
    pub(super) fn call(
        &self,
        runtime: &mut dyn HostCallRuntime<Profile>,
    ) -> Result<char, HostCallError> {
        let (state, arguments) = runtime.scalar_context();
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for char {
    fn descriptor() -> crate::host::HostTypeDescriptor {
        <Self as HostAbiType>::descriptor()
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunction::utf_codepoint(
            HostUtfCodepointFunction {
                implementation: Arc::new(function),
            },
        ))
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
