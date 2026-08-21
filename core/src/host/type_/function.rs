use super::{
    HostAbiType, HostAbiTypeSequence, HostCustomIdentity, HostCustomTypeSchema, HostSchemaType,
    HostType, HostTypeDescriptor, HostTypeSequence, private,
};
use crate::host::{HostCallable, HostScopedValue, HostValue};
use crate::provider_support::HostOpaqueFunctionType;
use std::collections::HashSet;
use std::marker::PhantomData;

/// The host ABI type for a Gleam function with a recursive argument sequence.
pub struct HostFunctionType<Arguments, Return>(PhantomData<(Arguments, Return)>);

impl<Arguments, Return> private::Sealed for HostFunctionType<Arguments, Return>
where
    Arguments: HostTypeSequence,
    Return: HostType,
{
}

impl<Arguments, Return> private::Sealed for HostOpaqueFunctionType<Arguments, Return>
where
    Arguments: HostTypeSequence,
    Return: HostType,
{
}

impl<Arguments, Return> HostType for HostFunctionType<Arguments, Return>
where
    Arguments: HostTypeSequence,
    Return: HostType,
{
    type Value<'call> = HostCallable<'call, Arguments, Return>;
}

impl<Arguments, Return> HostType for HostOpaqueFunctionType<Arguments, Return>
where
    Arguments: HostTypeSequence,
    Return: HostType,
{
    type Value<'call> = HostValue<'call, Self>;
}

impl<Arguments, Return> private::Abi for HostFunctionType<Arguments, Return>
where
    Arguments: HostAbiTypeSequence,
    Return: HostAbiType,
{
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Function {
            arguments: <Arguments as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
            return_: Box::new(<Return as HostAbiType>::descriptor()),
        }
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Function {
            arguments: <Arguments as HostAbiTypeSequence>::schema_types().into_boxed_slice(),
            return_: Box::new(<Return as HostAbiType>::schema_type()),
        }
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Arguments as HostAbiTypeSequence>::collect_custom_schemas(output, visited);
        <Return as HostAbiType>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Function(value.token)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        HostCallable::new(runtime.function_token(token))
    }
}

impl<Arguments, Return> private::Abi for HostOpaqueFunctionType<Arguments, Return>
where
    Arguments: HostAbiTypeSequence,
    Return: HostAbiType,
{
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::OpaqueFunction {
            arguments: <Arguments as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
            return_: Box::new(<Return as HostAbiType>::descriptor()),
        }
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Function {
            arguments: <Arguments as HostAbiTypeSequence>::schema_types().into_boxed_slice(),
            return_: Box::new(<Return as HostAbiType>::schema_type()),
        }
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Arguments as HostAbiTypeSequence>::collect_custom_schemas(output, visited);
        <Return as HostAbiType>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Value(value.token)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        _runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        HostValue::new(token)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFunctionType, HostOpaqueFunctionType};
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostAbiType, HostCallable, HostFunctionToken, HostSchemaType, HostScopedValue,
        HostTypeDescriptor, HostTypeList, HostTypeListEnd, HostValueFamily, HostValueToken,
    };
    use num_bigint::BigInt;

    #[test]
    fn function_abi_preserves_its_signature_and_runtime_token() {
        type Arguments = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;
        type Function = HostFunctionType<Arguments, bool>;

        assert_eq!(
            <Function as HostAbiType>::descriptor(),
            HostTypeDescriptor::Function {
                arguments: vec![HostTypeDescriptor::Int, HostTypeDescriptor::Bool]
                    .into_boxed_slice(),
                return_: Box::new(HostTypeDescriptor::Bool),
            },
        );
        assert_eq!(
            <Function as HostAbiType>::schema_type(),
            HostSchemaType::function(
                [HostSchemaType::Int, HostSchemaType::Bool],
                HostSchemaType::Bool,
            ),
        );
        assert_eq!(
            <Function as HostAbiType>::into_scoped(HostCallable::new(HostFunctionToken(4))),
            HostScopedValue::Function(HostFunctionToken(4)),
        );

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let token = HostValueToken {
            family: HostValueFamily::Function,
            index: 0,
        };
        assert_eq!(
            crate::host::type_::from_token::<Function, TestHostProfile>(&runtime, token).token,
            HostFunctionToken(0),
        );
    }

    #[test]
    fn opaque_function_abi_preserves_its_signature_without_invocation_capability() {
        type Arguments = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;
        type Function = HostOpaqueFunctionType<Arguments, bool>;

        assert_eq!(
            <Function as HostAbiType>::descriptor(),
            HostTypeDescriptor::OpaqueFunction {
                arguments: vec![HostTypeDescriptor::Int, HostTypeDescriptor::Bool]
                    .into_boxed_slice(),
                return_: Box::new(HostTypeDescriptor::Bool),
            },
        );
        assert_eq!(
            <Function as HostAbiType>::schema_type(),
            HostSchemaType::function(
                [HostSchemaType::Int, HostSchemaType::Bool],
                HostSchemaType::Bool,
            ),
        );

        let token = HostValueToken {
            family: HostValueFamily::Function,
            index: 3,
        };
        let value = crate::host::HostValue::<Function>::new(token);
        assert_eq!(
            <Function as HostAbiType>::into_scoped(value),
            HostScopedValue::Value(token),
        );

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        assert_eq!(
            crate::host::type_::from_token::<Function, TestHostProfile>(&runtime, token).token,
            token,
        );
    }
}
