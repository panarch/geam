use super::{
    HostAbiType, HostCustomSchemaId, HostCustomTypeSchema, HostSchemaType, HostType,
    HostTypeDescriptor, private,
};
use crate::host::{HostScopedValue, HostTokenRuntime, HostValueToken};
use std::collections::HashSet;
use std::marker::PhantomData;

/// Registers an exact restoration target without granting construction rights.
///
/// Include this type in the host function's permission sequence and select its
/// [`crate::HostRestoration`] with `HostConstruction::restoration`.
pub struct HostRestoredType<Type>(PhantomData<Type>);

impl<Type: HostType> private::Sealed for HostRestoredType<Type> {}

impl<Type: HostType> HostType for HostRestoredType<Type> {
    type Value<'call> = Type::Value<'call>;
}

impl<Type: HostType> private::Abi for HostRestoredType<Type> {
    fn descriptor() -> HostTypeDescriptor {
        <Type as HostAbiType>::descriptor()
    }

    fn schema_type() -> HostSchemaType {
        <Type as HostAbiType>::schema_type()
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomSchemaId>,
    ) {
        <Type as HostAbiType>::collect_custom_schemas(output, visited);
    }

    fn collect_permissions(
        _constructions: &mut Vec<HostTypeDescriptor>,
        restorations: &mut Vec<HostTypeDescriptor>,
    ) {
        restorations.push(<Type as HostAbiType>::descriptor());
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        <Type as private::Abi>::into_scoped(value)
    }

    fn from_token<'call, Runtime: HostTokenRuntime + ?Sized>(
        runtime: &Runtime,
        token: HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        <Type as private::Abi>::from_token(runtime, token)
    }
}

#[cfg(test)]
mod tests {
    use super::HostRestoredType;
    use crate::host::test::StatelessTestProvider as Provider;
    use crate::host::{
        HostAbiType, HostAbiTypeSequence, HostCall, HostCallCompletion, HostCallError,
        HostConstructions, HostCustomConstructorDefinition, HostCustomConstructorList,
        HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
        HostCustomSchema, HostCustomType, HostProviderModule, HostProviderSet,
        HostRetainedCustomSchema, HostRetainedCustomType, HostScopedValue, HostType,
        HostTypeDescriptor, HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter,
        HostValue, StatelessHostProfile,
    };
    use crate::{
        CustomTypeName, HostProviderLinkReason, HostedExecution, ModuleSource, PackageSource,
        PlanError, Value,
    };
    use num_bigint::BigInt;

    struct Secret;
    struct Constructor;
    struct Field;
    impl HostCustomSchema for Secret {
        const PACKAGE: &'static str = "producer";
        const MODULE: &'static str = "handles";
        const NAME: &'static str = "Secret";
        const PARAMETER_COUNT: usize = 0;
        const SHARED: bool = true;
        type Constructors = HostCustomConstructorList<Constructor, HostCustomConstructorListEnd>;
    }
    impl HostCustomConstructorDefinition for Constructor {
        const NAME: &'static str = "Secret";
        type Fields = HostCustomFieldList<Field, HostCustomFieldListEnd>;
    }
    impl HostCustomField for Field {
        const LABEL: Option<&'static str> = None;
        type Type = BigInt;
    }
    struct RetainedSecret;
    impl HostRetainedCustomSchema for RetainedSecret {
        const PACKAGE: &'static str = "producer";
        const MODULE: &'static str = "handles";
        const NAME: &'static str = "Secret";
        const PARAMETER_COUNT: usize = 0;
    }
    type Permission<Type> = HostTypeList<HostRestoredType<Type>, HostTypeListEnd>;
    type Argument = HostTypeParameter<0>;

    fn restore<'call, Type: HostType>(
        mut call: HostCall<'call, StatelessHostProfile, Provider, bool>,
        permissions: HostConstructions<'call, Permission<Type>>,
        value: HostValue<'call, Argument>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        let native = call.native_value::<Argument>(value);
        assert_eq!(native.kind(), crate::provider::advanced::NativeKind::Opaque);
        assert!(native.index(0).is_none());
        let restored =
            call.restore_native(&permissions.at::<HostTypeIndex0>().restoration(), &native);
        Ok(call.return_value(restored.is_some()))
    }

    #[test]
    fn nominal_restoration_does_not_grant_an_unregistered_full_schema() {
        let packages = || {
            [
                PackageSource::new(
                    "producer",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "handles",
                        "handles.gleam",
                        "pub opaque type Secret { Secret(Int) } pub fn make() { Secret(42) }",
                    )],
                ),
                PackageSource::new(
                    "consumer",
                    ["producer"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "import handles\n@external(erlang, \"native\", \"restore\") fn restore(value: a) -> Bool\npub fn main() { restore(handles.make()) }",
                    )],
                ),
            ]
        };
        let producer = || {
            HostProviderModule::new("producer", "handles")
                .unwrap()
                .with_retained_custom_type::<RetainedSecret>()
                .unwrap()
        };
        let full = HostProviderModule::new("consumer", "main").unwrap()
            .with_scoped_function_and_constructions::<Provider, (Argument,), bool, Permission<HostCustomType<Secret>>, _>("restore", restore::<HostCustomType<Secret>>).unwrap();
        let typed = crate::compile_typed_host_program(
            "consumer",
            "main",
            packages(),
            HostProviderSet::from_providers([producer(), full]).unwrap(),
        )
        .unwrap();
        assert_eq!(
            crate::plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "consumer".into(),
                module: "main".into(),
                function: "restore".into(),
                reason: Box::new(HostProviderLinkReason::MissingSharedCustomType {
                    custom_type: CustomTypeName::new(
                        "producer".into(),
                        "handles".into(),
                        "Secret".into()
                    )
                }),
            })
        );

        type Target = HostRetainedCustomType<RetainedSecret>;
        let consumer = HostProviderModule::new("consumer", "main").unwrap()
            .with_scoped_function_and_constructions::<Provider, (Argument,), bool, Permission<Target>, _>("restore", restore::<Target>).unwrap();
        let typed = crate::compile_typed_host_program(
            "consumer",
            "main",
            packages(),
            HostProviderSet::from_providers([producer(), consumer]).unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn restoration_registration_preserves_exact_abi_and_separates_permission_roles() {
        use crate::host::function::CallArguments;
        use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
        use crate::host::{HostSchemaType, HostValueFamily, HostValueToken};
        type Target = HostRestoredType<BigInt>;
        type Permissions = HostTypeList<bool, Permission<BigInt>>;
        assert_eq!(
            Permissions::permissions(),
            (
                vec![HostTypeDescriptor::Bool],
                vec![HostTypeDescriptor::Int]
            )
        );
        assert_eq!(Target::descriptor(), HostTypeDescriptor::Int);
        assert_eq!(Target::schema_type(), HostSchemaType::Int);
        assert_eq!(
            Target::into_scoped(42.into()),
            HostScopedValue::Int(42.into())
        );
        let mut state = TestRunState::default();
        let runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(vec![42.into()], Vec::new()));
        let token = HostValueToken {
            family: HostValueFamily::Int,
            index: 0,
        };
        assert_eq!(
            crate::host::type_::from_token::<Target, TestHostProfile>(&runtime, token),
            BigInt::from(0)
        );
    }
}
