use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
use std::sync::Arc;

pub(crate) struct HostNilFunction<Profile: HostProfile> {
    implementation: Arc<HostCallback<Profile, ()>>,
}

impl<Profile: HostProfile> Clone for HostNilFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            implementation: Arc::clone(&self.implementation),
        }
    }
}

impl<Profile: HostProfile> HostNilFunction<Profile> {
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<(), HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for () {
    fn type_() -> HostValueType {
        HostValueType::Nil
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Nil(HostNilFunction {
            implementation: Arc::new(function),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFunctionImplementation, HostNilFunction, HostReturn};
    use crate::host::StatelessHostProfile;
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};

    #[test]
    fn nil_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<()>();
        assert_eq!(<() as HostReturn>::type_(), HostValueType::Nil);
        let implementation =
            <() as HostReturn>::implementation::<StatelessHostProfile>(move |(), arguments| {
                arguments.nil(slot);
                Ok(())
            });
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            1,
        );

        assert_eq!(
            nil_implementation(implementation).call(&mut (), &arguments),
            Ok(()),
        );
    }

    #[test]
    #[should_panic(expected = "() return should create a Nil implementation")]
    fn nil_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(true);
        let implementation = <bool as HostReturn>::implementation::<StatelessHostProfile>(callback);
        nil_implementation(implementation);
    }

    fn nil_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostNilFunction<StatelessHostProfile> {
        let HostFunctionImplementation::Nil(implementation) = implementation else {
            panic!("() return should create a Nil implementation");
        };
        implementation
    }
}
