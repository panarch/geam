use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
use ecow::EcoString;
use std::sync::Arc;

pub(crate) struct HostStringFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, EcoString>>,
}

impl<Profile: HostProfile> Clone for HostStringFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostStringFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<EcoString, HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for EcoString {
    fn type_() -> HostValueType {
        HostValueType::String
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::String(HostStringFunction {
            implementation: Arc::new(function),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFunctionImplementation, HostReturn, HostStringFunction};
    use crate::host::StatelessHostProfile;
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};
    use ecow::EcoString;

    #[test]
    fn string_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<EcoString>();
        assert_eq!(<EcoString as HostReturn>::type_(), HostValueType::String,);
        let implementation = <EcoString as HostReturn>::implementation::<StatelessHostProfile>(
            move |(), arguments| Ok(format!("{}!", arguments.string(slot)).into()),
        );
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            vec!["hello".into()],
            Vec::new(),
            Vec::new(),
            0,
        );

        assert_eq!(
            string_implementation(implementation).call(&mut (), &arguments),
            Ok(EcoString::from("hello!")),
        );
    }

    #[test]
    #[should_panic(expected = "EcoString return should create a String implementation")]
    fn string_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(true);
        let implementation = <bool as HostReturn>::implementation::<StatelessHostProfile>(callback);
        string_implementation(implementation);
    }

    fn string_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostStringFunction<StatelessHostProfile> {
        let HostFunctionImplementation::String(implementation) = implementation else {
            panic!("EcoString return should create a String implementation");
        };
        implementation
    }
}
