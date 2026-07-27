use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
use std::sync::Arc;

pub(crate) struct HostUtfCodepointFunction<Profile: HostProfile> {
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
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<char, HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for char {
    fn type_() -> HostValueType {
        HostValueType::UtfCodepoint
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::UtfCodepoint(HostUtfCodepointFunction {
            implementation: Arc::new(function),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFunctionImplementation, HostReturn, HostUtfCodepointFunction};
    use crate::host::StatelessHostProfile;
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};

    #[test]
    fn utf_codepoint_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<char>();
        assert_eq!(<char as HostReturn>::type_(), HostValueType::UtfCodepoint,);
        let implementation =
            <char as HostReturn>::implementation::<StatelessHostProfile>(move |(), arguments| {
                Ok(arguments.utf_codepoint(slot))
            });
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec!['A'],
            0,
        );

        assert_eq!(
            utf_codepoint_implementation(implementation).call(&mut (), &arguments),
            Ok('A'),
        );
    }

    #[test]
    #[should_panic(expected = "char return should create a UtfCodepoint implementation")]
    fn utf_codepoint_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(true);
        let implementation = <bool as HostReturn>::implementation::<StatelessHostProfile>(callback);
        utf_codepoint_implementation(implementation);
    }

    fn utf_codepoint_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostUtfCodepointFunction<StatelessHostProfile> {
        let HostFunctionImplementation::UtfCodepoint(implementation) = implementation else {
            panic!("char return should create a UtfCodepoint implementation");
        };
        implementation
    }
}
