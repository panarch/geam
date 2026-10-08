use super::{HostRetainedCustomSchema, HostRetainedCustomType};
use crate::host::type_::{HostAbiTypeSequence, HostCustomSchemaId, private};
use crate::host::{
    HostCustom, HostCustomTypeSchema, HostSchemaType, HostScopedValue, HostType,
    HostTypeDescriptor, HostTypeSequence,
};
use std::collections::HashSet;

impl<Schema, Arguments> private::Sealed for HostRetainedCustomType<Schema, Arguments>
where
    Schema: HostRetainedCustomSchema,
    Arguments: HostTypeSequence,
{
}

impl<Schema, Arguments> HostType for HostRetainedCustomType<Schema, Arguments>
where
    Schema: HostRetainedCustomSchema,
    Arguments: HostTypeSequence,
{
    type Value<'call> = HostCustom<'call, Self>;
}

impl<Schema, Arguments> private::Abi for HostRetainedCustomType<Schema, Arguments>
where
    Schema: HostRetainedCustomSchema,
    Arguments: HostAbiTypeSequence,
{
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Custom {
            schema: HostCustomTypeSchema::retained::<Schema>(),
            arguments: <Arguments as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
        }
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::custom(
            Schema::PACKAGE,
            Schema::MODULE,
            Schema::NAME,
            <Arguments as HostAbiTypeSequence>::schema_types(),
        )
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomSchemaId>,
    ) {
        let schema = HostCustomTypeSchema::retained::<Schema>();
        if !output.contains(&schema) {
            output.push(schema);
        }
        <Arguments as HostAbiTypeSequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Custom(value.token)
    }

    fn from_token<'call, Runtime: crate::host::HostTokenRuntime + ?Sized>(
        runtime: &Runtime,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        HostCustom::new(runtime.custom_token(token))
    }
}

#[cfg(test)]
mod tests {
    use super::{HostRetainedCustomSchema, HostRetainedCustomType};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostCustom, HostProviderModule,
        HostProviderSet, HostTypeParameter, HostValue,
    };
    use crate::{HostedExecution, ModuleSource, PackageSource, StatelessHostProfile, Value};

    use crate::host::test::StatelessTestProvider as Provider;
    use crate::runtime::NativeKind;

    struct Schema;
    impl HostRetainedCustomSchema for Schema {
        const PACKAGE: &'static str = "producer";
        const MODULE: &'static str = "handles";
        const NAME: &'static str = "Handle";
        const PARAMETER_COUNT: usize = 1;
    }
    type Handle<T> = HostRetainedCustomType<Schema, crate::HostTypeList<T, crate::HostTypeListEnd>>;
    type GenericHandle = Handle<HostTypeParameter<0>>;

    fn preserve<'call>(
        call: HostCall<'call, StatelessHostProfile, Provider, GenericHandle>,
        value: HostCustom<'call, GenericHandle>,
    ) -> Result<HostCallCompletion<'call, GenericHandle>, HostCallError> {
        let native = call.native_value::<GenericHandle>(value);
        assert_eq!(native.kind(), NativeKind::Opaque);
        assert!(native.index(0).is_none());
        Ok(call.return_value(value))
    }

    fn generic<'call>(
        call: HostCall<'call, StatelessHostProfile, Provider, HostTypeParameter<0>>,
        value: HostValue<'call, HostTypeParameter<0>>,
    ) -> Result<HostCallCompletion<'call, HostTypeParameter<0>>, HostCallError> {
        let native = call.native_value::<HostTypeParameter<0>>(value);
        assert_eq!(native.kind(), NativeKind::Opaque);
        assert!(native.index(1).is_none());
        Ok(call.return_value(value))
    }

    #[test]
    fn nominal_retention_does_not_depend_on_the_producers_private_layout() {
        for producer_source in [
            r#"
pub opaque type Handle(a) { Handle(value: a, secret: fn(Int) -> Int) }
pub fn make(value: a) { Handle(value, fn(item) { item + 1 }) }
pub fn get(handle: Handle(a)) { handle.value }
"#,
            r#"
pub opaque type Handle(a) { Handle(secret: #(Bool, String), value: a) }
pub fn make(value: a) { Handle(#(True, "private"), value) }
pub fn get(handle: Handle(a)) { handle.value }
"#,
        ] {
            let producer = HostProviderModule::new("producer", "handles")
                .unwrap()
                .with_retained_custom_type::<Schema>()
                .unwrap();
            let consumer = HostProviderModule::new("consumer", "main")
                .unwrap()
                .with_scoped_function::<Provider, (GenericHandle,), GenericHandle, _>(
                    "preserve", preserve,
                )
                .unwrap();
            let typed = crate::compile_typed_host_program("consumer", "main", [
                PackageSource::new("producer", Vec::<&str>::new(), [ModuleSource::new("handles", "handles.gleam", producer_source)]),
                PackageSource::new("consumer", ["producer"], [ModuleSource::new("main", "main.gleam", r#"
import handles
@external(erlang, "native", "preserve")
fn preserve(value: handles.Handle(a)) -> handles.Handle(a)
pub fn main() { #(handles.get(preserve(handles.make(42))), handles.get(preserve(handles.make("text")))) }
"#)])
            ], HostProviderSet::from_providers([producer, consumer]).unwrap()).unwrap();
            let mut execution =
                HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                    .unwrap();
            assert_eq!(
                crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
                Value::Tuple(vec![Value::Int(42.into()), Value::String("text".into())])
            );
        }
    }

    #[test]
    fn nominal_abi_and_custom_fields_share_the_producer_identity_and_type_arguments() {
        use crate::host::type_::custom::field::{CustomFieldType, ResolveCustomFieldType};
        use crate::host::{
            HostAbiType, HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor, HostTypeList,
            HostTypeListEnd,
        };
        use num_bigint::BigInt;
        use std::collections::HashSet;

        type Nominal = Handle<BigInt>;
        let expected =
            HostSchemaType::custom("producer", "handles", "Handle", [HostSchemaType::Int]);
        assert_eq!(<Nominal as HostAbiType>::schema_type(), expected);
        assert_eq!(<Nominal as CustomFieldType>::schema_type(), expected);
        type Field = Handle<crate::HostCustomTypeArgument<crate::HostTypeIndex0>>;
        type Resolved =
            <Field as ResolveCustomFieldType<HostTypeList<BigInt, HostTypeListEnd>>>::Type;
        assert_eq!(
            Resolved::descriptor(),
            HostTypeDescriptor::Custom {
                schema: HostCustomTypeSchema::retained::<Schema>(),
                arguments: Box::new([HostTypeDescriptor::Int]),
            }
        );
        let mut schemas = Vec::new();
        let mut visited = HashSet::new();
        for _ in 0..2 {
            <Nominal as CustomFieldType>::collect_custom_schemas(&mut schemas, &mut visited);
        }
        assert_eq!(schemas, [HostCustomTypeSchema::retained::<Schema>()]);
    }

    #[test]
    fn generic_preservation_needs_no_nominal_registration_and_grants_no_view() {
        let consumer = HostProviderModule::new("consumer", "main")
            .unwrap()
            .with_scoped_function::<Provider, (HostTypeParameter<0>,), HostTypeParameter<0>, _>(
                "generic", generic,
            )
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "consumer",
            "main",
            [PackageSource::new(
                "consumer",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
pub opaque type Secret { Secret(Int) }
@external(erlang, "native", "generic")
fn generic(value: a) -> a
pub fn main() { let assert Secret(value) = generic(Secret(42)) value }
"#,
                )],
            )],
            HostProviderSet::from_providers([consumer]).unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            Value::Int(42.into())
        );
    }
}
