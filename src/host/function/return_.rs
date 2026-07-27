use super::HostValueType;
use super::argument::HostCallArguments;
use num_bigint::BigInt;
use std::sync::Arc;

type HostCallback<Return> = dyn Fn(&dyn HostCallArguments) -> Return + Send + Sync;

#[derive(Clone)]
pub(crate) enum HostFunctionImplementation {
    Int(HostIntFunction),
    Bool(HostBoolFunction),
}

#[derive(Clone)]
pub(crate) struct HostIntFunction {
    implementation: Arc<HostCallback<BigInt>>,
}

#[derive(Clone)]
pub(crate) struct HostBoolFunction {
    implementation: Arc<HostCallback<bool>>,
}

pub(super) trait HostReturn: Sized {
    fn type_() -> HostValueType;

    fn implementation(
        function: impl Fn(&dyn HostCallArguments) -> Self + Send + Sync + 'static,
    ) -> HostFunctionImplementation;
}

impl HostIntFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> BigInt {
        (self.implementation)(arguments)
    }
}

impl HostBoolFunction {
    pub(crate) fn call(&self, arguments: &dyn HostCallArguments) -> bool {
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
    use super::{HostBoolFunction, HostFunctionImplementation, HostIntFunction, HostReturn};
    use crate::host::function::HostValueType;
    use crate::host::function::argument::{
        HostBoolArgumentSlot, HostCallArguments, HostIntArgumentSlot, HostParameterLayout,
    };
    use num_bigint::BigInt;

    struct EmptyArguments;

    impl HostCallArguments for EmptyArguments {
        fn int(&self, _slot: HostIntArgumentSlot) -> BigInt {
            BigInt::from(0)
        }

        fn bool(&self, _slot: HostBoolArgumentSlot) -> bool {
            false
        }
    }

    #[test]
    fn int_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<BigInt>();
        assert_eq!(<BigInt as HostReturn>::type_(), HostValueType::Int);
        let implementation =
            <BigInt as HostReturn>::implementation(move |arguments| arguments.int(slot) + 42);

        assert_eq!(
            int_implementation(implementation).call(&EmptyArguments),
            BigInt::from(42),
        );
    }

    #[test]
    fn bool_return_owns_typed_callback_and_family() {
        let mut layout = HostParameterLayout::default();
        let slot = layout.register::<bool>();
        assert_eq!(<bool as HostReturn>::type_(), HostValueType::Bool);
        let implementation =
            <bool as HostReturn>::implementation(move |arguments| !arguments.bool(slot));

        assert!(bool_implementation(implementation).call(&EmptyArguments));
    }

    #[test]
    #[should_panic(expected = "BigInt return should create an Int implementation")]
    fn int_return_shape_guard_is_visible() {
        let callback = |_: &dyn HostCallArguments| true;
        assert!(callback(&EmptyArguments));
        let implementation = <bool as HostReturn>::implementation(callback);
        int_implementation(implementation);
    }

    #[test]
    #[should_panic(expected = "bool return should create a Bool implementation")]
    fn bool_return_shape_guard_is_visible() {
        let callback = |_: &dyn HostCallArguments| BigInt::from(1);
        assert_eq!(callback(&EmptyArguments), BigInt::from(1));
        let implementation = <BigInt as HostReturn>::implementation(callback);
        bool_implementation(implementation);
    }

    fn int_implementation(implementation: HostFunctionImplementation) -> HostIntFunction {
        let HostFunctionImplementation::Int(implementation) = implementation else {
            panic!("BigInt return should create an Int implementation");
        };
        implementation
    }

    fn bool_implementation(implementation: HostFunctionImplementation) -> HostBoolFunction {
        let HostFunctionImplementation::Bool(implementation) = implementation else {
            panic!("bool return should create a Bool implementation");
        };
        implementation
    }
}
