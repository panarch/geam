use super::{HostSchemaType, HostType, HostTypeDescriptor, private};
use crate::host::{HostScopedValue, HostValue};

/// A generic parameter in a host function's Gleam type scheme.
pub struct HostTypeParameter<const INDEX: usize>;

impl<const INDEX: usize> private::Sealed for HostTypeParameter<INDEX> {}

impl<const INDEX: usize> HostType for HostTypeParameter<INDEX> {
    type Value<'call> = HostValue<'call, Self>;
}

impl<const INDEX: usize> private::Abi for HostTypeParameter<INDEX> {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Parameter(INDEX)
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Parameter(INDEX)
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
    use super::HostTypeParameter;
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostAbiType, HostSchemaType, HostScopedValue, HostTypeDescriptor, HostValue,
        HostValueFamily, HostValueToken,
    };

    #[test]
    fn type_parameter_abi_preserves_its_index_and_runtime_token() {
        type Parameter = HostTypeParameter<2>;

        assert_eq!(
            <Parameter as HostAbiType>::descriptor(),
            HostTypeDescriptor::Parameter(2),
        );
        assert_eq!(
            <Parameter as HostAbiType>::schema_type(),
            HostSchemaType::Parameter(2),
        );
        let token = HostValueToken {
            family: HostValueFamily::Bool,
            index: 3,
        };
        assert_eq!(
            <Parameter as HostAbiType>::into_scoped(HostValue::new(token)),
            HostScopedValue::Value(token),
        );

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        assert_eq!(
            crate::host::type_::from_token::<Parameter, TestHostProfile>(&runtime, token).token,
            token,
        );
    }
}
