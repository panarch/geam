use super::{
    HostCallback, HostFunctionImplementation, HostReturn, HostValueFunctionImplementation,
};
use crate::BitArrayValue;
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use crate::host::{HostCallError, HostProfile};
use std::sync::Arc;

pub(crate) struct HostBitArrayFunction<Profile: HostProfile> {
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
    pub(crate) fn call(
        &self,
        state: &mut Profile::RunState,
        arguments: &dyn HostCallArguments,
    ) -> Result<BitArrayValue, HostCallError> {
        (self.implementation)(state, arguments)
    }
}

impl HostReturn for BitArrayValue {
    fn type_() -> HostValueType {
        HostValueType::BitArray
    }

    fn implementation<Profile: HostProfile>(
        function: impl Fn(&mut Profile::RunState, &dyn HostCallArguments) -> Result<Self, HostCallError>
        + Send
        + Sync
        + 'static,
    ) -> HostFunctionImplementation<Profile> {
        HostFunctionImplementation::Value(HostValueFunctionImplementation::BitArray(
            HostBitArrayFunction {
                implementation: Arc::new(function),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostBitArrayFunction, HostFunctionImplementation, HostReturn,
        HostValueFunctionImplementation,
    };
    use crate::BitArrayValue;
    use crate::host::StatelessHostProfile;
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};

    #[test]
    fn bit_array_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<BitArrayValue>();
        assert_eq!(
            <BitArrayValue as HostReturn>::type_(),
            HostValueType::BitArray,
        );
        let implementation = <BitArrayValue as HostReturn>::implementation::<StatelessHostProfile>(
            move |(), arguments| Ok(arguments.bit_array(slot)),
        );
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            vec![BitArrayValue::from_bytes(vec![0xa5])],
            Vec::new(),
            0,
        );

        assert_eq!(
            bit_array_implementation(implementation).call(&mut (), &arguments),
            Ok(BitArrayValue::from_bytes(vec![0xa5])),
        );
    }

    #[test]
    #[should_panic(expected = "BitArrayValue return should create a BitArray implementation")]
    fn bit_array_return_shape_guard_is_visible() {
        let callback = |(): &mut (), _: &dyn HostCallArguments| Ok(true);
        let implementation = <bool as HostReturn>::implementation::<StatelessHostProfile>(callback);
        bit_array_implementation(implementation);
    }

    fn bit_array_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostBitArrayFunction<StatelessHostProfile> {
        let HostFunctionImplementation::Value(HostValueFunctionImplementation::BitArray(
            implementation,
        )) = implementation
        else {
            panic!("BitArrayValue return should create a BitArray implementation");
        };
        implementation
    }
}
