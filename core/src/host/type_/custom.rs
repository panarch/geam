mod field;

pub use field::HostCustomTypeArgument;

use super::{
    HostAbiTypeSequence, HostCustomIdentity, HostType, HostTypeDescriptor, HostTypeList,
    HostTypeListEnd, HostTypeSequence, private,
};
use crate::host::{HostCustom, HostScopedValue};
use crate::provider_support::{SoleHostCustomConstructor, private as provider_private};
use ecow::EcoString;
use field::{CustomFieldType, ResolveCustomFieldType};
use std::collections::HashSet;
use std::marker::PhantomData;

/// An ordinary Gleam custom type and its concrete type arguments.
pub struct HostCustomType<Schema, Arguments = HostTypeListEnd>(PhantomData<(Schema, Arguments)>);

/// A constructor selected at `Index` from `Custom`'s sealed constructor list.
pub struct HostCustomConstructorAt<Custom, Index, Definition>(
    PhantomData<(Custom, Index, Definition)>,
);

/// One custom constructor definition followed by the remaining definitions.
pub struct HostCustomConstructorList<Head, Tail>(PhantomData<(Head, Tail)>);

/// The end of an ordered custom constructor list.
pub struct HostCustomConstructorListEnd;

/// One custom field definition followed by the remaining definitions.
pub struct HostCustomFieldList<Head, Tail>(PhantomData<(Head, Tail)>);

/// The end of an ordered custom field list.
pub struct HostCustomFieldListEnd;

/// The first position in a custom constructor list.
pub struct HostCustomIndex0;

/// The position following `Index` in a custom constructor list.
pub struct HostCustomIndexNext<Index>(PhantomData<Index>);

/// The complete source schema for one ordinary Gleam custom type.
pub trait HostCustomSchema: Send + Sync + 'static {
    const PACKAGE: &'static str;
    const MODULE: &'static str;
    const NAME: &'static str;
    const PARAMETER_COUNT: usize;

    type Constructors: HostCustomConstructorSequence;
}

/// A constructor proven to occur in the declared schema for `Custom`.
///
/// This trait is sealed. Constructors must be selected with
/// [`HostCustomConstructorAt`], so safe user code cannot fabricate a runtime
/// constructor index.
///
/// ```compile_fail
/// use geam_core::{
///     HostCustomConstructor, HostCustomConstructorListEnd, HostCustomSchema, HostCustomType,
///     HostTypeListEnd,
/// };
///
/// struct Schema;
///
/// impl HostCustomSchema for Schema {
///     const PACKAGE: &'static str = "application";
///     const MODULE: &'static str = "main";
///     const NAME: &'static str = "Thing";
///     const PARAMETER_COUNT: usize = 0;
///
///     type Constructors = HostCustomConstructorListEnd;
/// }
///
/// type Thing = HostCustomType<Schema>;
/// struct Fabricated;
///
/// impl HostCustomConstructor for Fabricated {
///     type Custom = Thing;
///     type Fields = HostTypeListEnd;
/// }
/// ```
#[allow(private_bounds)]
pub trait HostCustomConstructor: private::CustomConstructor + Send + Sync + 'static {
    type Custom: HostType;

    /// Constructor fields after substituting `Custom`'s concrete type arguments.
    type Fields: HostTypeSequence;
}

/// The source name and ordered fields of one custom constructor.
pub trait HostCustomConstructorDefinition: Send + Sync + 'static {
    const NAME: &'static str;

    type Fields: HostCustomFieldSequence;
}

/// A sealed recursive sequence of custom constructor definitions.
#[allow(private_bounds)]
pub trait HostCustomConstructorSequence:
    private::CustomConstructors + Send + Sync + 'static
{
}

impl<Constructors> HostCustomConstructorSequence for Constructors where
    Constructors: private::CustomConstructors + Send + Sync + 'static
{
}

/// The source label and type expression of one custom constructor field.
///
/// [`HostCustomTypeArgument`] refers to a parameter declared by the enclosing
/// [`HostCustomSchema`]. Function-level generics continue to use
/// [`super::HostTypeParameter`].
#[allow(private_bounds)]
pub trait HostCustomField: Send + Sync + 'static {
    const LABEL: Option<&'static str>;

    type Type: CustomFieldType;
}

/// A sealed recursive sequence of custom constructor fields.
#[allow(private_bounds)]
pub trait HostCustomFieldSequence: private::CustomFields + Send + Sync + 'static {}

impl<Fields> HostCustomFieldSequence for Fields where
    Fields: private::CustomFields + Send + Sync + 'static
{
}

/// The source-facing schema of an ordinary custom type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCustomTypeSchema {
    package: EcoString,
    module: EcoString,
    name: EcoString,
    parameter_count: usize,
    constructors: Box<[HostCustomConstructorSchema]>,
}

/// The source-facing schema of one custom constructor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCustomConstructorSchema {
    name: EcoString,
    fields: Box<[HostCustomFieldSchema]>,
}

/// The source-facing schema of one custom constructor field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCustomFieldSchema {
    label: Option<EcoString>,
    type_: HostSchemaType,
}

/// A recursive source-facing type used while validating custom schemas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostSchemaType {
    Parameter(usize),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List(Box<HostSchemaType>),
    Tuple(Box<[HostSchemaType]>),
    Function {
        arguments: Box<[HostSchemaType]>,
        return_: Box<HostSchemaType>,
    },
    Custom {
        package: EcoString,
        module: EcoString,
        name: EcoString,
        arguments: Box<[HostSchemaType]>,
    },
    External {
        schema: crate::host::HostExternalTypeSchema,
        arguments: Box<[HostSchemaType]>,
    },
}

impl HostCustomTypeSchema {
    pub fn of<Schema: HostCustomSchema>() -> Self {
        Self::new(
            Schema::PACKAGE,
            Schema::MODULE,
            Schema::NAME,
            Schema::PARAMETER_COUNT,
            <Schema::Constructors as private::CustomConstructors>::schemas(),
        )
    }

    pub fn new(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
        name: impl Into<EcoString>,
        parameter_count: usize,
        constructors: impl IntoIterator<Item = HostCustomConstructorSchema>,
    ) -> Self {
        Self {
            package: package.into(),
            module: module.into(),
            name: name.into(),
            parameter_count,
            constructors: constructors
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn parameter_count(&self) -> usize {
        self.parameter_count
    }

    pub fn constructors(&self) -> &[HostCustomConstructorSchema] {
        &self.constructors
    }
}

impl HostCustomConstructorSchema {
    pub fn new(
        name: impl Into<EcoString>,
        fields: impl IntoIterator<Item = HostCustomFieldSchema>,
    ) -> Self {
        Self {
            name: name.into(),
            fields: fields.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn fields(&self) -> &[HostCustomFieldSchema] {
        &self.fields
    }
}

impl HostCustomFieldSchema {
    pub fn new(label: Option<impl Into<EcoString>>, type_: HostSchemaType) -> Self {
        Self {
            label: label.map(Into::into),
            type_,
        }
    }

    pub fn label(&self) -> Option<&EcoString> {
        self.label.as_ref()
    }

    pub fn type_(&self) -> &HostSchemaType {
        &self.type_
    }
}

impl HostSchemaType {
    pub fn parameter(index: usize) -> Self {
        Self::Parameter(index)
    }

    pub fn list(item: Self) -> Self {
        Self::List(Box::new(item))
    }

    pub fn tuple(elements: impl IntoIterator<Item = Self>) -> Self {
        Self::Tuple(elements.into_iter().collect::<Vec<_>>().into_boxed_slice())
    }

    pub fn function(arguments: impl IntoIterator<Item = Self>, return_: Self) -> Self {
        Self::Function {
            arguments: arguments.into_iter().collect::<Vec<_>>().into_boxed_slice(),
            return_: Box::new(return_),
        }
    }

    pub fn custom(
        package: impl Into<EcoString>,
        module: impl Into<EcoString>,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = Self>,
    ) -> Self {
        Self::Custom {
            package: package.into(),
            module: module.into(),
            name: name.into(),
            arguments: arguments.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub(crate) fn collect_external_schemas(
        &self,
        output: &mut Vec<crate::host::HostExternalTypeSchema>,
        visited: &mut HashSet<(EcoString, EcoString, EcoString)>,
    ) {
        match self {
            Self::List(item) => item.collect_external_schemas(output, visited),
            Self::Tuple(elements) => {
                for element in elements {
                    element.collect_external_schemas(output, visited);
                }
            }
            Self::Function { arguments, return_ } => {
                for argument in arguments {
                    argument.collect_external_schemas(output, visited);
                }
                return_.collect_external_schemas(output, visited);
            }
            Self::Custom { arguments, .. } => {
                for argument in arguments {
                    argument.collect_external_schemas(output, visited);
                }
            }
            Self::External { schema, arguments } => {
                let identity = (
                    schema.package().clone(),
                    schema.module().clone(),
                    schema.name().clone(),
                );
                if visited.insert(identity) {
                    output.push(schema.clone());
                }
                for argument in arguments {
                    argument.collect_external_schemas(output, visited);
                }
            }
            Self::Parameter(_)
            | Self::Int
            | Self::Float
            | Self::String
            | Self::BitArray
            | Self::UtfCodepoint
            | Self::Bool
            | Self::Nil => {}
        }
    }
}

impl<Head, Tail> private::CustomConstructors for HostCustomConstructorList<Head, Tail>
where
    Head: HostCustomConstructorDefinition,
    Tail: HostCustomConstructorSequence,
{
    fn schemas() -> Vec<HostCustomConstructorSchema> {
        let mut constructors = vec![HostCustomConstructorSchema::new(
            Head::NAME,
            <Head::Fields as private::CustomFields>::schemas(),
        )];
        constructors.extend(<Tail as private::CustomConstructors>::schemas());
        constructors
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Head::Fields as private::CustomFields>::collect_custom_schemas(output, visited);
        <Tail as private::CustomConstructors>::collect_custom_schemas(output, visited);
    }
}

impl private::CustomConstructors for HostCustomConstructorListEnd {
    fn schemas() -> Vec<HostCustomConstructorSchema> {
        Vec::new()
    }

    fn collect_custom_schemas(
        _output: &mut Vec<HostCustomTypeSchema>,
        _visited: &mut HashSet<HostCustomIdentity>,
    ) {
    }
}

impl<Head, Tail> private::ConstructorAt<HostCustomIndex0, Head>
    for HostCustomConstructorList<Head, Tail>
where
    Head: HostCustomConstructorDefinition,
    Tail: HostCustomConstructorSequence,
{
    fn index() -> usize {
        0
    }
}

impl<Head, Tail, Index, Definition> private::ConstructorAt<HostCustomIndexNext<Index>, Definition>
    for HostCustomConstructorList<Head, Tail>
where
    Head: HostCustomConstructorDefinition,
    Tail: HostCustomConstructorSequence + private::ConstructorAt<Index, Definition>,
    Definition: HostCustomConstructorDefinition,
    Index: Send + Sync + 'static,
{
    fn index() -> usize {
        1 + <Tail as private::ConstructorAt<Index, Definition>>::index()
    }
}

impl<Schema, Arguments, Index, Definition> private::CustomConstructor
    for HostCustomConstructorAt<HostCustomType<Schema, Arguments>, Index, Definition>
where
    Schema: HostCustomSchema,
    Arguments: HostTypeSequence,
    Index: Send + Sync + 'static,
    Definition: HostCustomConstructorDefinition,
    Schema::Constructors: private::ConstructorAt<Index, Definition>,
{
    fn index() -> usize {
        <Schema::Constructors as private::ConstructorAt<Index, Definition>>::index()
    }
}

impl<Schema, Arguments, Index, Definition> HostCustomConstructor
    for HostCustomConstructorAt<HostCustomType<Schema, Arguments>, Index, Definition>
where
    Schema: HostCustomSchema,
    Arguments: HostTypeSequence,
    Index: Send + Sync + 'static,
    Definition: HostCustomConstructorDefinition,
    Definition::Fields: ResolveCustomFields<Arguments>,
    Schema::Constructors: private::ConstructorAt<Index, Definition>,
{
    type Custom = HostCustomType<Schema, Arguments>;
    type Fields = <Definition::Fields as ResolveCustomFields<Arguments>>::Fields;
}

impl<Schema, Arguments, Definition> SoleHostCustomConstructor
    for HostCustomConstructorAt<HostCustomType<Schema, Arguments>, HostCustomIndex0, Definition>
where
    Schema: HostCustomSchema<
        Constructors = HostCustomConstructorList<Definition, HostCustomConstructorListEnd>,
    >,
    Arguments: HostTypeSequence,
    Definition: HostCustomConstructorDefinition,
    Definition::Fields: ResolveCustomFields<Arguments>,
{
}

impl<Schema, Arguments, Definition> provider_private::SoleHostCustomConstructor
    for HostCustomConstructorAt<HostCustomType<Schema, Arguments>, HostCustomIndex0, Definition>
where
    Schema: HostCustomSchema<
        Constructors = HostCustomConstructorList<Definition, HostCustomConstructorListEnd>,
    >,
    Arguments: HostTypeSequence,
    Definition: HostCustomConstructorDefinition,
    Definition::Fields: ResolveCustomFields<Arguments>,
{
}

impl private::CustomFields for HostCustomFieldListEnd {
    fn schemas() -> Vec<HostCustomFieldSchema> {
        Vec::new()
    }

    fn collect_custom_schemas(
        _output: &mut Vec<HostCustomTypeSchema>,
        _visited: &mut HashSet<HostCustomIdentity>,
    ) {
    }
}

#[doc(hidden)]
pub trait ResolveCustomFields<Arguments>: HostCustomFieldSequence
where
    Arguments: HostTypeSequence,
{
    type Fields: HostTypeSequence;
}

impl<Arguments> ResolveCustomFields<Arguments> for HostCustomFieldListEnd
where
    Arguments: HostTypeSequence,
{
    type Fields = HostTypeListEnd;
}

impl<Arguments, Head, Tail> ResolveCustomFields<Arguments> for HostCustomFieldList<Head, Tail>
where
    Arguments: HostTypeSequence,
    Head: HostCustomField,
    Head::Type: ResolveCustomFieldType<Arguments>,
    Tail: HostCustomFieldSequence + ResolveCustomFields<Arguments>,
{
    type Fields = HostTypeList<
        <Head::Type as ResolveCustomFieldType<Arguments>>::Type,
        <Tail as ResolveCustomFields<Arguments>>::Fields,
    >;
}

impl<Head, Tail> private::CustomFields for HostCustomFieldList<Head, Tail>
where
    Head: HostCustomField,
    Tail: HostCustomFieldSequence,
{
    fn schemas() -> Vec<HostCustomFieldSchema> {
        let mut fields = vec![HostCustomFieldSchema::new(
            Head::LABEL,
            <Head::Type as CustomFieldType>::schema_type(),
        )];
        fields.extend(<Tail as private::CustomFields>::schemas());
        fields
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Head::Type as CustomFieldType>::collect_custom_schemas(output, visited);
        <Tail as private::CustomFields>::collect_custom_schemas(output, visited);
    }
}

pub(super) fn collect_custom_type_schema<Schema: HostCustomSchema>(
    output: &mut Vec<HostCustomTypeSchema>,
    visited: &mut HashSet<HostCustomIdentity>,
) {
    let schema = HostCustomTypeSchema::of::<Schema>();
    if !output.contains(&schema) {
        output.push(schema);
    }
    let identity = (
        EcoString::from(Schema::PACKAGE),
        EcoString::from(Schema::MODULE),
        EcoString::from(Schema::NAME),
    );
    if visited.insert(identity) {
        <Schema::Constructors as private::CustomConstructors>::collect_custom_schemas(
            output, visited,
        );
    }
}

impl<Schema, Arguments> private::Sealed for HostCustomType<Schema, Arguments>
where
    Schema: HostCustomSchema,
    Arguments: HostTypeSequence,
{
}

impl<Schema, Arguments> HostType for HostCustomType<Schema, Arguments>
where
    Schema: HostCustomSchema,
    Arguments: HostTypeSequence,
{
    type Value<'call> = HostCustom<'call, Self>;
}

impl<Schema, Arguments> private::Abi for HostCustomType<Schema, Arguments>
where
    Schema: HostCustomSchema,
    Arguments: HostAbiTypeSequence,
{
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Custom {
            schema: HostCustomTypeSchema::of::<Schema>(),
            arguments: <Arguments as HostAbiTypeSequence>::descriptors().into_boxed_slice(),
        }
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Custom {
            package: Schema::PACKAGE.into(),
            module: Schema::MODULE.into(),
            name: Schema::NAME.into(),
            arguments: <Arguments as HostAbiTypeSequence>::schema_types().into_boxed_slice(),
        }
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        collect_custom_type_schema::<Schema>(output, visited);
        <Arguments as HostAbiTypeSequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Custom(value.token)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        HostCustom::new(runtime.custom_token(token))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomConstructorSchema, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
        HostCustomFieldSchema, HostCustomSchema, HostCustomType, HostCustomTypeArgument,
        HostCustomTypeSchema,
    };
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostAbiType, HostCustom, HostCustomToken, HostExternalTypeSchema, HostSchemaType,
        HostScopedValue, HostTypeDescriptor, HostTypeIndex0, HostTypeList, HostTypeListEnd,
        HostTypeParameter, HostValueFamily, HostValueToken,
    };

    struct BoxedValueField;

    impl HostCustomField for BoxedValueField {
        const LABEL: Option<&'static str> = Some("value");

        type Type = HostCustomTypeArgument<HostTypeIndex0>;
    }

    struct BoxedConstructor;

    impl HostCustomConstructorDefinition for BoxedConstructor {
        const NAME: &'static str = "Boxed";

        type Fields = HostCustomFieldList<BoxedValueField, HostCustomFieldListEnd>;
    }

    struct BoxedSchema;

    impl HostCustomSchema for BoxedSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/box";
        const NAME: &'static str = "Boxed";
        const PARAMETER_COUNT: usize = 1;

        type Constructors =
            HostCustomConstructorList<BoxedConstructor, HostCustomConstructorListEnd>;
    }

    #[test]
    fn custom_abi_preserves_nominal_schema_arguments_and_runtime_token() {
        type Arguments = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;
        type Boxed = HostCustomType<BoxedSchema, Arguments>;

        let schema = HostCustomTypeSchema::new(
            "domain",
            "domain/box",
            "Boxed",
            1,
            [HostCustomConstructorSchema::new(
                "Boxed",
                [HostCustomFieldSchema::new(
                    Some("value"),
                    HostSchemaType::parameter(0),
                )],
            )],
        );
        assert_eq!(HostCustomTypeSchema::of::<BoxedSchema>(), schema);
        assert_eq!(
            <Boxed as HostAbiType>::descriptor(),
            HostTypeDescriptor::Custom {
                schema: schema.clone(),
                arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
            },
        );
        assert_eq!(
            <Boxed as HostAbiType>::schema_type(),
            HostSchemaType::custom(
                "domain",
                "domain/box",
                "Boxed",
                [HostSchemaType::parameter(0)],
            ),
        );
        assert_eq!(
            <Boxed as HostAbiType>::into_scoped(HostCustom::new(HostCustomToken(4))),
            HostScopedValue::Custom(HostCustomToken(4)),
        );

        let mut schemas = Vec::new();
        let mut visited = std::collections::HashSet::new();
        <Boxed as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
        assert_eq!(schemas, [schema]);

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let token = HostValueToken {
            family: HostValueFamily::Custom,
            index: 0,
        };
        assert_eq!(
            crate::host::type_::from_token::<Boxed, TestHostProfile>(&runtime, token).token,
            HostCustomToken(0),
        );
    }

    #[test]
    fn external_schema_collection_follows_nested_host_schema_types() {
        let resource = HostExternalTypeSchema::new("domain", "domain/resource", "Resource", 0);
        let box_ = HostExternalTypeSchema::new("domain", "domain/box", "Box", 1);
        let type_ = HostSchemaType::tuple([
            HostSchemaType::External {
                schema: resource.clone(),
                arguments: Vec::new().into_boxed_slice(),
            },
            HostSchemaType::function(
                [
                    HostSchemaType::list(HostSchemaType::External {
                        schema: box_.clone(),
                        arguments: vec![HostSchemaType::Int].into_boxed_slice(),
                    }),
                    HostSchemaType::External {
                        schema: resource.clone(),
                        arguments: Vec::new().into_boxed_slice(),
                    },
                ],
                HostSchemaType::custom(
                    "domain",
                    "domain/wrapper",
                    "Wrapper",
                    [
                        HostSchemaType::External {
                            schema: box_.clone(),
                            arguments: vec![HostSchemaType::Bool].into_boxed_slice(),
                        },
                        HostSchemaType::Nil,
                    ],
                ),
            ),
        ]);
        let mut schemas = Vec::new();
        let mut visited = std::collections::HashSet::new();

        type_.collect_external_schemas(&mut schemas, &mut visited);

        assert_eq!(schemas, [resource, box_]);
    }
}
