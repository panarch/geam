use super::{HostAbiTypeSequence, HostType, HostTypeDescriptor, HostTypeSequence, private};
use crate::host::{
    HostExternal, HostExternalSchema, HostExternalType, HostExternalTypeSchema, HostScopedValue,
};

impl<Schema, Arguments> HostType for HostExternalType<Schema, Arguments>
where
    Schema: HostExternalSchema,
    Arguments: HostTypeSequence,
{
    type Value<'call> = HostExternal<'call, Self>;
}

impl<Schema, Arguments> private::Sealed for HostExternalType<Schema, Arguments>
where
    Schema: HostExternalSchema,
    Arguments: HostTypeSequence,
{
}

impl<Schema, Arguments> private::Abi for HostExternalType<Schema, Arguments>
where
    Schema: HostExternalSchema,
    Arguments: HostAbiTypeSequence,
{
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::External {
            schema: HostExternalTypeSchema::of::<Schema>(),
            arguments: <Arguments as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
        }
    }

    fn schema_type() -> super::HostSchemaType {
        super::HostSchemaType::External {
            schema: HostExternalTypeSchema::of::<Schema>(),
            arguments: <Arguments as HostAbiTypeSequence>::schema_types().into_boxed_slice(),
        }
    }

    fn collect_custom_schemas(
        output: &mut Vec<super::HostCustomTypeSchema>,
        visited: &mut std::collections::HashSet<super::HostCustomIdentity>,
    ) {
        <Arguments as HostAbiTypeSequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::External(value.token)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        HostExternal::new(runtime.external_token(token))
    }
}

#[cfg(test)]
mod tests {
    use super::HostExternalType;
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostAbiType, HostExternal, HostExternalSchema, HostExternalToken, HostExternalTypeSchema,
        HostSchemaType, HostScopedValue, HostTypeDescriptor, HostTypeList, HostTypeListEnd,
        HostTypeParameter, HostValueFamily, HostValueToken,
    };

    struct BoxSchema;

    impl HostExternalSchema for BoxSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/box";
        const NAME: &'static str = "Box";
        const PARAMETER_COUNT: usize = 1;
    }

    #[test]
    fn external_abi_preserves_nominal_schema_arguments_and_runtime_token() {
        type Arguments = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;
        type Boxed = HostExternalType<BoxSchema, Arguments>;

        let schema = HostExternalTypeSchema::new("domain", "domain/box", "Box", 1);
        assert_eq!(HostExternalTypeSchema::of::<BoxSchema>(), schema);
        assert_eq!(
            <Boxed as HostAbiType>::descriptor(),
            HostTypeDescriptor::External {
                schema: schema.clone(),
                arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
            },
        );
        assert_eq!(
            <Boxed as HostAbiType>::schema_type(),
            HostSchemaType::External {
                schema,
                arguments: vec![HostSchemaType::parameter(0)].into_boxed_slice(),
            },
        );
        assert_eq!(
            <Boxed as HostAbiType>::into_scoped(HostExternal::new(HostExternalToken(4))),
            HostScopedValue::External(HostExternalToken(4)),
        );

        let mut schemas = Vec::new();
        let mut visited = std::collections::HashSet::new();
        <Boxed as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
        assert!(schemas.is_empty());

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let token = HostValueToken {
            family: HostValueFamily::External,
            index: 0,
        };
        assert_eq!(
            crate::host::type_::from_token::<Boxed, TestHostProfile>(&runtime, token).token,
            HostExternalToken(0),
        );
    }
}
