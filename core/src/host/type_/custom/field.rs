use super::{HostCustomIdentity, HostCustomSchema, HostCustomType, HostCustomTypeSchema};
use super::{HostSchemaType, collect_custom_type_schema};
use crate::BitArrayValue;
use crate::host::{
    HostAbiType, HostExternalSchema, HostExternalType, HostExternalTypeSchema, HostFunctionType,
    HostListType, HostTupleType, HostType, HostTypeAt, HostTypeIndex0, HostTypeIndexNext,
    HostTypeList, HostTypeListEnd, HostTypeSequence,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashSet;
use std::marker::PhantomData;

/// One type argument from the ordinary custom schema currently being defined.
///
/// Use [`HostTypeIndex0`] for the first schema argument and nest
/// [`HostTypeIndexNext`] for later arguments. Unlike [`crate::HostTypeParameter`],
/// this marker is resolved against the concrete arguments of the enclosing
/// [`HostCustomType`].
pub struct HostCustomTypeArgument<Index>(PhantomData<Index>);

#[doc(hidden)]
pub trait CustomFieldType: Send + Sync + 'static {
    fn schema_type() -> HostSchemaType;

    fn collect_custom_schemas(
        _output: &mut Vec<HostCustomTypeSchema>,
        _visited: &mut HashSet<HostCustomIdentity>,
    ) {
    }
}

#[doc(hidden)]
pub trait ResolveCustomFieldType<Arguments>: CustomFieldType
where
    Arguments: HostTypeSequence,
{
    type Type: HostType;
}

#[doc(hidden)]
pub trait CustomFieldTypeSequence: Send + Sync + 'static {
    fn schema_types() -> Vec<HostSchemaType>;

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    );
}

#[doc(hidden)]
pub trait ResolveCustomFieldTypeSequence<Arguments>: CustomFieldTypeSequence
where
    Arguments: HostTypeSequence,
{
    type Types: HostTypeSequence;
}

trait CustomTypeArgumentIndex: Send + Sync + 'static {
    const INDEX: usize;
}

trait AtomicCustomFieldType: HostAbiType {}

impl AtomicCustomFieldType for BigInt {}
impl AtomicCustomFieldType for f64 {}
impl AtomicCustomFieldType for EcoString {}
impl AtomicCustomFieldType for BitArrayValue {}
impl AtomicCustomFieldType for char {}
impl AtomicCustomFieldType for bool {}
impl AtomicCustomFieldType for () {}

impl<Type> CustomFieldType for Type
where
    Type: AtomicCustomFieldType,
{
    fn schema_type() -> HostSchemaType {
        <Type as HostAbiType>::schema_type()
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Type as HostAbiType>::collect_custom_schemas(output, visited);
    }
}

impl<Arguments, Type> ResolveCustomFieldType<Arguments> for Type
where
    Arguments: HostTypeSequence,
    Type: AtomicCustomFieldType,
{
    type Type = Type;
}

impl CustomTypeArgumentIndex for HostTypeIndex0 {
    const INDEX: usize = 0;
}

impl<Index> CustomTypeArgumentIndex for HostTypeIndexNext<Index>
where
    Index: CustomTypeArgumentIndex,
{
    const INDEX: usize = 1 + Index::INDEX;
}

impl<Index> CustomFieldType for HostCustomTypeArgument<Index>
where
    Index: CustomTypeArgumentIndex,
{
    fn schema_type() -> HostSchemaType {
        HostSchemaType::Parameter(Index::INDEX)
    }
}

impl<Arguments, Index> ResolveCustomFieldType<Arguments> for HostCustomTypeArgument<Index>
where
    Arguments: HostTypeAt<Index>,
    Index: CustomTypeArgumentIndex,
{
    type Type = <Arguments as HostTypeAt<Index>>::Type;
}

impl<Item> CustomFieldType for HostListType<Item>
where
    Item: CustomFieldType,
{
    fn schema_type() -> HostSchemaType {
        HostSchemaType::list(Item::schema_type())
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        Item::collect_custom_schemas(output, visited);
    }
}

impl<Arguments, Item> ResolveCustomFieldType<Arguments> for HostListType<Item>
where
    Arguments: HostTypeSequence,
    Item: ResolveCustomFieldType<Arguments>,
{
    type Type = HostListType<Item::Type>;
}

impl<Elements> CustomFieldType for HostTupleType<Elements>
where
    Elements: CustomFieldTypeSequence,
{
    fn schema_type() -> HostSchemaType {
        HostSchemaType::tuple(Elements::schema_types())
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        Elements::collect_custom_schemas(output, visited);
    }
}

impl<Arguments, Elements> ResolveCustomFieldType<Arguments> for HostTupleType<Elements>
where
    Arguments: HostTypeSequence,
    Elements: ResolveCustomFieldTypeSequence<Arguments>,
{
    type Type = HostTupleType<Elements::Types>;
}

impl<FunctionArguments, Return> CustomFieldType for HostFunctionType<FunctionArguments, Return>
where
    FunctionArguments: CustomFieldTypeSequence,
    Return: CustomFieldType,
{
    fn schema_type() -> HostSchemaType {
        HostSchemaType::function(FunctionArguments::schema_types(), Return::schema_type())
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        FunctionArguments::collect_custom_schemas(output, visited);
        Return::collect_custom_schemas(output, visited);
    }
}

impl<Arguments, FunctionArguments, Return> ResolveCustomFieldType<Arguments>
    for HostFunctionType<FunctionArguments, Return>
where
    Arguments: HostTypeSequence,
    FunctionArguments: ResolveCustomFieldTypeSequence<Arguments>,
    Return: ResolveCustomFieldType<Arguments>,
{
    type Type = HostFunctionType<FunctionArguments::Types, Return::Type>;
}

impl<Schema, TypeArguments> CustomFieldType for HostCustomType<Schema, TypeArguments>
where
    Schema: HostCustomSchema,
    TypeArguments: CustomFieldTypeSequence,
{
    fn schema_type() -> HostSchemaType {
        HostSchemaType::custom(
            Schema::PACKAGE,
            Schema::MODULE,
            Schema::NAME,
            TypeArguments::schema_types(),
        )
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        collect_custom_type_schema::<Schema>(output, visited);
        TypeArguments::collect_custom_schemas(output, visited);
    }
}

impl<Arguments, Schema, TypeArguments> ResolveCustomFieldType<Arguments>
    for HostCustomType<Schema, TypeArguments>
where
    Arguments: HostTypeSequence,
    Schema: HostCustomSchema,
    TypeArguments: ResolveCustomFieldTypeSequence<Arguments>,
{
    type Type = HostCustomType<Schema, TypeArguments::Types>;
}

impl<Schema, TypeArguments> CustomFieldType for HostExternalType<Schema, TypeArguments>
where
    Schema: HostExternalSchema,
    TypeArguments: CustomFieldTypeSequence,
{
    fn schema_type() -> HostSchemaType {
        HostSchemaType::External {
            schema: HostExternalTypeSchema::of::<Schema>(),
            arguments: TypeArguments::schema_types().into_boxed_slice(),
        }
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        TypeArguments::collect_custom_schemas(output, visited);
    }
}

impl<Arguments, Schema, TypeArguments> ResolveCustomFieldType<Arguments>
    for HostExternalType<Schema, TypeArguments>
where
    Arguments: HostTypeSequence,
    Schema: HostExternalSchema,
    TypeArguments: ResolveCustomFieldTypeSequence<Arguments>,
{
    type Type = HostExternalType<Schema, TypeArguments::Types>;
}

impl CustomFieldTypeSequence for HostTypeListEnd {
    fn schema_types() -> Vec<HostSchemaType> {
        Vec::new()
    }

    fn collect_custom_schemas(
        _output: &mut Vec<HostCustomTypeSchema>,
        _visited: &mut HashSet<HostCustomIdentity>,
    ) {
    }
}

impl<Arguments> ResolveCustomFieldTypeSequence<Arguments> for HostTypeListEnd
where
    Arguments: HostTypeSequence,
{
    type Types = HostTypeListEnd;
}

impl<Head, Tail> CustomFieldTypeSequence for HostTypeList<Head, Tail>
where
    Head: CustomFieldType,
    Tail: CustomFieldTypeSequence,
{
    fn schema_types() -> Vec<HostSchemaType> {
        let mut types = vec![Head::schema_type()];
        types.extend(Tail::schema_types());
        types
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        Head::collect_custom_schemas(output, visited);
        Tail::collect_custom_schemas(output, visited);
    }
}

impl<Arguments, Head, Tail> ResolveCustomFieldTypeSequence<Arguments> for HostTypeList<Head, Tail>
where
    Arguments: HostTypeSequence,
    Head: ResolveCustomFieldType<Arguments>,
    Tail: ResolveCustomFieldTypeSequence<Arguments>,
{
    type Types = HostTypeList<Head::Type, Tail::Types>;
}

#[cfg(test)]
mod tests {
    use super::HostCustomTypeArgument;
    use crate::host::{
        HostAbiType, HostAbiTypeSequence, HostCustomConstructor, HostCustomConstructorAt,
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomConstructorSchema, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
        HostCustomFieldSchema, HostCustomIndex0, HostCustomSchema, HostCustomType,
        HostCustomTypeSchema, HostExternalSchema, HostExternalType, HostExternalTypeSchema,
        HostFunctionType, HostListType, HostSchemaType, HostTupleType, HostTypeDescriptor,
        HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    };
    use num_bigint::BigInt;
    use std::collections::HashSet;

    type FirstArgument = HostCustomTypeArgument<HostTypeIndex0>;
    type SecondArgument = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;

    struct NestedValueField;

    impl HostCustomField for NestedValueField {
        const LABEL: Option<&'static str> = Some("value");

        type Type = FirstArgument;
    }

    struct NestedConstructor;

    impl HostCustomConstructorDefinition for NestedConstructor {
        const NAME: &'static str = "Nested";

        type Fields = HostCustomFieldList<NestedValueField, HostCustomFieldListEnd>;
    }

    struct NestedSchema;

    impl HostCustomSchema for NestedSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/types";
        const NAME: &'static str = "Nested";
        const PARAMETER_COUNT: usize = 1;

        type Constructors =
            HostCustomConstructorList<NestedConstructor, HostCustomConstructorListEnd>;
    }

    struct ExternalSchema;

    impl HostExternalSchema for ExternalSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/external";
        const NAME: &'static str = "External";
        const PARAMETER_COUNT: usize = 1;
    }

    struct DirectField;
    struct ListField;
    struct TupleField;
    struct FunctionField;
    struct CustomField;
    struct ExternalField;

    impl HostCustomField for DirectField {
        const LABEL: Option<&'static str> = Some("direct");
        type Type = FirstArgument;
    }

    impl HostCustomField for ListField {
        const LABEL: Option<&'static str> = Some("list");
        type Type = HostListType<FirstArgument>;
    }

    impl HostCustomField for TupleField {
        const LABEL: Option<&'static str> = Some("tuple");
        type Type =
            HostTupleType<HostTypeList<SecondArgument, HostTypeList<bool, HostTypeListEnd>>>;
    }

    impl HostCustomField for FunctionField {
        const LABEL: Option<&'static str> = Some("function");
        type Type = HostFunctionType<HostTypeList<SecondArgument, HostTypeListEnd>, FirstArgument>;
    }

    impl HostCustomField for CustomField {
        const LABEL: Option<&'static str> = Some("custom");
        type Type = HostCustomType<NestedSchema, HostTypeList<FirstArgument, HostTypeListEnd>>;
    }

    impl HostCustomField for ExternalField {
        const LABEL: Option<&'static str> = Some("external");
        type Type = HostExternalType<ExternalSchema, HostTypeList<SecondArgument, HostTypeListEnd>>;
    }

    type ConstructorFields = HostCustomFieldList<
        DirectField,
        HostCustomFieldList<
            ListField,
            HostCustomFieldList<
                TupleField,
                HostCustomFieldList<
                    FunctionField,
                    HostCustomFieldList<
                        CustomField,
                        HostCustomFieldList<ExternalField, HostCustomFieldListEnd>,
                    >,
                >,
            >,
        >,
    >;

    struct Constructor;

    impl HostCustomConstructorDefinition for Constructor {
        const NAME: &'static str = "Container";
        type Fields = ConstructorFields;
    }

    struct ContainerSchema;

    impl HostCustomSchema for ContainerSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/types";
        const NAME: &'static str = "Container";
        const PARAMETER_COUNT: usize = 2;

        type Constructors = HostCustomConstructorList<Constructor, HostCustomConstructorListEnd>;
    }

    #[test]
    fn custom_field_types_resolve_schema_arguments_through_every_compound_family() {
        type Arguments = HostTypeList<BigInt, HostTypeList<(), HostTypeListEnd>>;
        type Container = HostCustomType<ContainerSchema, Arguments>;
        type ContainerConstructor =
            HostCustomConstructorAt<Container, HostCustomIndex0, Constructor>;
        type Fields = <ContainerConstructor as HostCustomConstructor>::Fields;

        let parameter_0 = HostSchemaType::parameter(0);
        let parameter_1 = HostSchemaType::parameter(1);
        assert_eq!(
            HostCustomTypeSchema::of::<ContainerSchema>(),
            HostCustomTypeSchema::new(
                "domain",
                "domain/types",
                "Container",
                2,
                [HostCustomConstructorSchema::new(
                    "Container",
                    [
                        HostCustomFieldSchema::new(Some("direct"), parameter_0.clone()),
                        HostCustomFieldSchema::new(
                            Some("list"),
                            HostSchemaType::list(parameter_0.clone()),
                        ),
                        HostCustomFieldSchema::new(
                            Some("tuple"),
                            HostSchemaType::tuple([parameter_1.clone(), HostSchemaType::Bool]),
                        ),
                        HostCustomFieldSchema::new(
                            Some("function"),
                            HostSchemaType::function([parameter_1.clone()], parameter_0.clone()),
                        ),
                        HostCustomFieldSchema::new(
                            Some("custom"),
                            HostSchemaType::custom(
                                "domain",
                                "domain/types",
                                "Nested",
                                [parameter_0],
                            ),
                        ),
                        HostCustomFieldSchema::new(
                            Some("external"),
                            HostSchemaType::External {
                                schema: HostExternalTypeSchema::of::<ExternalSchema>(),
                                arguments: vec![parameter_1].into_boxed_slice(),
                            },
                        ),
                    ],
                )],
            ),
        );
        assert_eq!(
            <Fields as HostAbiTypeSequence>::descriptors(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int)),
                HostTypeDescriptor::Tuple(
                    vec![HostTypeDescriptor::Nil, HostTypeDescriptor::Bool].into_boxed_slice(),
                ),
                HostTypeDescriptor::Function {
                    arguments: vec![HostTypeDescriptor::Nil].into_boxed_slice(),
                    return_: Box::new(HostTypeDescriptor::Int),
                },
                HostTypeDescriptor::Custom {
                    schema: HostCustomTypeSchema::of::<NestedSchema>(),
                    arguments: vec![HostTypeDescriptor::Int].into_boxed_slice(),
                },
                HostTypeDescriptor::External {
                    schema: HostExternalTypeSchema::of::<ExternalSchema>(),
                    arguments: vec![HostTypeDescriptor::Nil].into_boxed_slice(),
                },
            ],
        );

        let mut schemas = Vec::new();
        let mut visited = HashSet::new();
        <Container as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
        assert_eq!(
            schemas,
            [
                HostCustomTypeSchema::of::<ContainerSchema>(),
                HostCustomTypeSchema::of::<NestedSchema>(),
            ],
        );
    }
}
