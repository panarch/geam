use crate::plan::{FunctionType, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFunctionSchema {
    name: EcoString,
    type_: FunctionType,
}

pub trait HostFunction<Arguments, Return>:
    private::HostFunction<Arguments, Return> + Send + Sync + 'static
{
}

impl<Function, Arguments, Return> HostFunction<Arguments, Return> for Function where
    Function: private::HostFunction<Arguments, Return> + Send + Sync + 'static
{
}

pub(crate) struct HostFunctionDefinition {
    schema: HostFunctionSchema,
    implementation: HostIntFunction,
}

#[derive(Clone)]
pub struct HostIntFunction {
    implementation: Arc<dyn Fn(BigInt, BigInt) -> BigInt + Send + Sync>,
}

impl HostFunctionSchema {
    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }
}

impl HostFunctionDefinition {
    pub(crate) fn new<Arguments, Return, Function>(name: EcoString, function: Function) -> Self
    where
        Function: HostFunction<Arguments, Return>,
    {
        Self {
            schema: HostFunctionSchema {
                name,
                type_: <Function as private::HostFunction<Arguments, Return>>::function_type(),
            },
            implementation:
                <Function as private::HostFunction<Arguments, Return>>::into_int_function(function),
        }
    }

    pub(crate) fn schema(&self) -> &HostFunctionSchema {
        &self.schema
    }

    pub(crate) fn into_parts(self) -> (HostFunctionSchema, HostIntFunction) {
        (self.schema, self.implementation)
    }
}

impl HostIntFunction {
    pub(crate) fn call(&self, left: BigInt, right: BigInt) -> BigInt {
        (self.implementation)(left, right)
    }
}

mod private {
    use super::{FunctionType, HostIntFunction, ValueType};
    use num_bigint::BigInt;
    use std::sync::Arc;

    pub trait HostFunction<Arguments, Return> {
        fn function_type() -> FunctionType;
        fn into_int_function(self) -> HostIntFunction;
    }

    impl<Function> HostFunction<(BigInt, BigInt), BigInt> for Function
    where
        Function: Fn(BigInt, BigInt) -> BigInt + Send + Sync + 'static,
    {
        fn function_type() -> FunctionType {
            FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int)
        }

        fn into_int_function(self) -> HostIntFunction {
            HostIntFunction {
                implementation: Arc::new(self),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFunctionDefinition, ValueType};
    use num_bigint::BigInt;

    #[test]
    fn int_binary_host_functions_own_exact_schema_and_implementation() {
        let definition =
            HostFunctionDefinition::new("subtract".into(), |left: BigInt, right: BigInt| {
                left - right
            });

        assert_eq!(definition.schema().name(), "subtract");
        assert_eq!(
            definition.schema().type_().argument_types(),
            [ValueType::Int, ValueType::Int],
        );
        assert_eq!(definition.schema().type_().return_(), &ValueType::Int);

        let (_, implementation) = definition.into_parts();
        assert_eq!(
            implementation.call(BigInt::from(10), BigInt::from(3)),
            BigInt::from(7),
        );
    }
}
