use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
use std::sync::Arc;

pub(crate) struct HostBoolFunction<Profile: HostProfile> {
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
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<bool, HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for bool {
    fn type_() -> HostValueType {
        HostValueType::Bool
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Bool(HostBoolFunction {
            implementation: Arc::new(function),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HostBoolFunction, HostFunctionImplementation, HostReturn};
    use crate::host::StatelessHostProfile;
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};
    use num_bigint::BigInt;

    #[test]
    fn bool_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<bool>();
        assert_eq!(<bool as HostReturn>::type_(), HostValueType::Bool);
        let implementation =
            <bool as HostReturn>::implementation::<StatelessHostProfile>(move |(), arguments| {
                Ok(!arguments.bool(slot))
            });
        let arguments = CallArguments::new(Vec::new(), vec![false]);

        assert_eq!(
            bool_implementation(implementation).call(&mut (), &arguments),
            Ok(true),
        );
    }

    #[test]
    #[should_panic(expected = "bool return should create a Bool implementation")]
    fn bool_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(BigInt::from(1));
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        assert_eq!(callback(&mut (), &arguments), Ok(BigInt::from(1)));
        let implementation =
            <BigInt as HostReturn>::implementation::<StatelessHostProfile>(callback);
        bool_implementation(implementation);
    }

    fn bool_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostBoolFunction<StatelessHostProfile> {
        let HostFunctionImplementation::Bool(implementation) = implementation else {
            panic!("bool return should create a Bool implementation");
        };
        implementation
    }
}
