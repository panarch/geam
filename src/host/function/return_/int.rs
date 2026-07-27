use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use num_bigint::BigInt;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct HostIntFunction {
    implementation: Arc<HostCallback<BigInt>>,
}

impl HostIntFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> BigInt {
        (self.implementation)(arguments)
    }
}

impl HostReturn for BigInt {
    fn type_() -> HostValueType {
        HostValueType::Int
    }

    fn implementation(
        function: impl Fn(&dyn HostCallArguments) -> Self + Send + Sync + 'static,
    ) -> HostFunctionImplementation {
        HostFunctionImplementation::Int(HostIntFunction {
            implementation: Arc::new(function),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFunctionImplementation, HostIntFunction, HostReturn};
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};
    use num_bigint::BigInt;

    #[test]
    fn int_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<BigInt>();
        assert_eq!(<BigInt as HostReturn>::type_(), HostValueType::Int);
        let implementation =
            <BigInt as HostReturn>::implementation(move |arguments| arguments.int(slot) + 42);
        let arguments = CallArguments::new(vec![BigInt::from(0)], Vec::new());

        assert_eq!(
            int_implementation(implementation).call(&arguments),
            BigInt::from(42),
        );
    }

    #[test]
    #[should_panic(expected = "BigInt return should create an Int implementation")]
    fn int_return_shape_guard_is_visible() {
        let callback = |_: &dyn HostCallArguments| true;
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        assert!(callback(&arguments));
        let implementation = <bool as HostReturn>::implementation(callback);
        int_implementation(implementation);
    }

    fn int_implementation(implementation: HostFunctionImplementation) -> HostIntFunction {
        let HostFunctionImplementation::Int(implementation) = implementation else {
            panic!("BigInt return should create an Int implementation");
        };
        implementation
    }
}
