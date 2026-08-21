use super::{
    HostAbiTypeSequence, HostCustomIdentity, HostCustomTypeSchema, HostSchemaType, HostType,
    HostTypeDescriptor, HostTypeSequence, private,
};
use crate::host::{HostScopedValue, HostTuple};
use std::collections::HashSet;
use std::marker::PhantomData;

/// The host ABI type for a tuple whose elements are a [`super::HostTypeList`].
pub struct HostTupleType<Elements>(PhantomData<Elements>);

impl<Elements: HostTypeSequence> private::Sealed for HostTupleType<Elements> {}

impl<Elements: HostTypeSequence> HostType for HostTupleType<Elements> {
    type Value<'call> = HostTuple<'call, Elements>;
}

impl<Elements: HostAbiTypeSequence> private::Abi for HostTupleType<Elements> {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Tuple(
            <Elements as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
        )
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Tuple(<Elements as HostAbiTypeSequence>::schema_types().into_boxed_slice())
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Elements as HostAbiTypeSequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Tuple(value.token)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        HostTuple::new(runtime.tuple_token(token))
    }
}

#[cfg(test)]
mod tests {
    use super::HostTupleType;
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostAbiType, HostSchemaType, HostScopedValue, HostTuple, HostTupleToken,
        HostTypeDescriptor, HostTypeList, HostTypeListEnd, HostValueFamily, HostValueToken,
    };
    use num_bigint::BigInt;

    #[test]
    fn tuple_abi_preserves_unbounded_element_order_and_runtime_token() {
        type Elements = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;
        type Tuple = HostTupleType<Elements>;

        assert_eq!(
            <Tuple as HostAbiType>::descriptor(),
            HostTypeDescriptor::Tuple(
                vec![HostTypeDescriptor::Int, HostTypeDescriptor::Bool].into_boxed_slice(),
            ),
        );
        assert_eq!(
            <Tuple as HostAbiType>::schema_type(),
            HostSchemaType::tuple([HostSchemaType::Int, HostSchemaType::Bool]),
        );
        assert_eq!(
            <Tuple as HostAbiType>::into_scoped(HostTuple::new(HostTupleToken(4))),
            HostScopedValue::Tuple(HostTupleToken(4)),
        );

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let token = HostValueToken {
            family: HostValueFamily::Tuple,
            index: 0,
        };
        assert_eq!(
            crate::host::type_::from_token::<Tuple, TestHostProfile>(&runtime, token).token,
            HostTupleToken(0),
        );
    }
}
