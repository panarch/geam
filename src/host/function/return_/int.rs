use super::{
    HostCallback, HostFunctionImplementation, HostReturn, HostValueFunctionImplementation,
};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
use num_bigint::BigInt;
use std::sync::Arc;

pub(crate) struct HostIntFunction<Profile: HostProfile> {
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
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<BigInt, HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for BigInt {
    fn type_() -> HostValueType {
        HostValueType::Int
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunctionImplementation::Int(HostIntFunction {
            implementation: Arc::new(function),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostFunctionImplementation, HostIntFunction, HostReturn, HostValueFunctionImplementation,
    };
    use crate::host::StatelessHostProfile;
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};
    use num_bigint::BigInt;

    #[test]
    fn int_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<BigInt>();
        assert_eq!(<BigInt as HostReturn>::type_(), HostValueType::Int);
        let implementation =
            <BigInt as HostReturn>::implementation::<StatelessHostProfile>(move |(), arguments| {
                Ok(arguments.int(slot) + 42)
            });
        let arguments = CallArguments::new(vec![BigInt::from(0)], Vec::new());

        assert_eq!(
            int_implementation(implementation).call(&mut (), &arguments),
            Ok(BigInt::from(42)),
        );
    }

    #[test]
    #[should_panic(expected = "BigInt return should create an Int implementation")]
    fn int_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(true);
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        assert_eq!(callback(&mut (), &arguments), Ok(true));
        let implementation = <bool as HostReturn>::implementation::<StatelessHostProfile>(callback);
        int_implementation(implementation);
    }

    fn int_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostIntFunction<StatelessHostProfile> {
        let HostFunctionImplementation::Value(HostValueFunctionImplementation::Int(implementation)) =
            implementation
        else {
            panic!("BigInt return should create an Int implementation");
        };
        implementation
    }
}
