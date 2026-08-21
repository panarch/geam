use super::{
    HostAbiType, HostCustomIdentity, HostCustomTypeSchema, HostSchemaType, HostType,
    HostTypeDescriptor, private,
};
use crate::host::{HostList, HostScopedValue};
use std::collections::HashSet;
use std::marker::PhantomData;

/// The host ABI type for `List(Item)`.
pub struct HostListType<Item>(PhantomData<Item>);

impl<Item: HostAbiType> private::Sealed for HostListType<Item> {}

impl<Item: HostAbiType> HostType for HostListType<Item> {
    type Value<'call> = HostList<'call, Item>;
}

impl<Item: HostAbiType> private::Abi for HostListType<Item> {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::List(Box::new(<Item as HostAbiType>::descriptor()))
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::List(Box::new(<Item as HostAbiType>::schema_type()))
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Item as HostAbiType>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::List(value.token)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        HostList::new(runtime.list_token(token))
    }
}

#[cfg(test)]
mod tests {
    use super::HostListType;
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostAbiType, HostList, HostListToken, HostSchemaType, HostScopedValue, HostTypeDescriptor,
        HostValueFamily, HostValueToken,
    };
    use num_bigint::BigInt;

    #[test]
    fn list_abi_preserves_its_item_type_and_runtime_token() {
        type List = HostListType<BigInt>;

        assert_eq!(
            <List as HostAbiType>::descriptor(),
            HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int)),
        );
        assert_eq!(
            <List as HostAbiType>::schema_type(),
            HostSchemaType::list(HostSchemaType::Int),
        );
        assert_eq!(
            <List as HostAbiType>::into_scoped(HostList::new(HostListToken::Stored(4))),
            HostScopedValue::List(HostListToken::Stored(4)),
        );

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let token = HostValueToken {
            family: HostValueFamily::List,
            index: 0,
        };
        assert_eq!(
            crate::host::type_::from_token::<List, TestHostProfile>(&runtime, token).token,
            HostListToken::Stored(0),
        );
    }
}
