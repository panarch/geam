use super::{HostCallback, HostFunctionImplementation, HostReturn};
use crate::host::function::HostValueType;
use crate::host::function::argument::HostCallArguments;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct HostBoolFunction {
    implementation: Arc<HostCallback<bool>>,
}

impl HostBoolFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> bool {
        (self.implementation)(arguments)
    }
}

impl HostReturn for bool {
    fn type_() -> HostValueType {
        HostValueType::Bool
    }

    fn implementation(
        function: impl Fn(&dyn HostCallArguments) -> Self + Send + Sync + 'static,
    ) -> HostFunctionImplementation {
        HostFunctionImplementation::Bool(HostBoolFunction {
            implementation: Arc::new(function),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HostBoolFunction, HostFunctionImplementation, HostReturn};
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{CallArguments, HostCallArguments, HostParameterLayout};
    use num_bigint::BigInt;

    #[test]
    fn bool_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<bool>();
        assert_eq!(<bool as HostReturn>::type_(), HostValueType::Bool);
        let implementation =
            <bool as HostReturn>::implementation(move |arguments| !arguments.bool(slot));
        let arguments = CallArguments::new(Vec::new(), vec![false]);

        assert!(bool_implementation(implementation).call(&arguments));
    }

    #[test]
    #[should_panic(expected = "bool return should create a Bool implementation")]
    fn bool_return_shape_guard_is_visible() {
        let callback = |_: &dyn HostCallArguments| BigInt::from(1);
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        assert_eq!(callback(&arguments), BigInt::from(1));
        let implementation = <BigInt as HostReturn>::implementation(callback);
        bool_implementation(implementation);
    }

    fn bool_implementation(implementation: HostFunctionImplementation) -> HostBoolFunction {
        let HostFunctionImplementation::Bool(implementation) = implementation else {
            panic!("bool return should create a Bool implementation");
        };
        implementation
    }
}
