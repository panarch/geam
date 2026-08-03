mod custom;
mod external;
mod function;
mod list;
mod parameter;
mod scalar;
mod sequence;
mod tuple;

pub use custom::{
    HostCustomConstructor, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomConstructorSchema,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostCustomTypeSchema, HostSchemaType,
};
pub use function::HostFunctionType;
pub use list::HostListType;
pub use parameter::HostTypeParameter;
pub use sequence::{
    HostTypeAt, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeSequence,
};
pub use tuple::HostTupleType;

pub(crate) use custom::SoleHostCustomConstructor;
pub(crate) use function::HostOpaqueFunctionType;
pub(crate) use sequence::HostAbiTypeSequence;

use super::HostScopedValue;
use ecow::EcoString;
use std::collections::{BTreeSet, HashSet};

/// A type accepted by the scoped host ABI.
#[allow(private_bounds)]
pub trait HostType: private::Sealed + private::Abi + Send + Sync + 'static {
    type Value<'call>: Clone;
}

type HostCustomIdentity = (EcoString, EcoString, EcoString);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HostTypeDescriptor {
    Parameter(usize),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List(Box<HostTypeDescriptor>),
    Tuple(Box<[HostTypeDescriptor]>),
    Function {
        arguments: Box<[HostTypeDescriptor]>,
        return_: Box<HostTypeDescriptor>,
    },
    OpaqueFunction {
        arguments: Box<[HostTypeDescriptor]>,
        return_: Box<HostTypeDescriptor>,
    },
    Custom {
        schema: HostCustomTypeSchema,
        arguments: Box<[HostTypeDescriptor]>,
    },
    External {
        schema: crate::host::HostExternalTypeSchema,
        arguments: Box<[HostTypeDescriptor]>,
    },
}

pub(crate) trait HostAbiType: HostType {
    fn descriptor() -> HostTypeDescriptor {
        <Self as private::Abi>::descriptor()
    }

    fn schema_type() -> HostSchemaType {
        <Self as private::Abi>::schema_type()
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Self as private::Abi>::collect_custom_schemas(output, visited);
    }

    fn into_scoped(value: Self::Value<'_>) -> HostScopedValue {
        <Self as private::Abi>::into_scoped(value)
    }
}

impl<Type: HostType> HostAbiType for Type {}

pub(crate) fn into_scoped<Type: HostType>(value: Type::Value<'_>) -> HostScopedValue {
    <Type as private::Abi>::into_scoped(value)
}

pub(crate) fn from_token<'call, Type: HostType, Profile: crate::host::HostProfile>(
    runtime: &dyn crate::host::HostCallRuntime<Profile>,
    token: crate::host::HostValueToken,
) -> Type::Value<'call> {
    <Type as private::Abi>::from_token(runtime, token)
}

pub(crate) fn into_scoped_values<Types: HostTypeSequence>(
    values: Types::Values<'_>,
    output: &mut Vec<HostScopedValue>,
) {
    <Types as private::Sequence>::into_scoped_values(values, output);
}

pub(crate) fn from_tokens<'call, Types: HostTypeSequence, Profile: crate::host::HostProfile>(
    runtime: &dyn crate::host::HostCallRuntime<Profile>,
    tokens: &[crate::host::HostValueToken],
) -> Types::Values<'call> {
    let mut index = 0;
    <Types as private::Sequence>::from_tokens(runtime, tokens, &mut index)
}

pub(crate) fn custom_constructor_index<Constructor: HostCustomConstructor>() -> usize {
    <Constructor as private::CustomConstructor>::index()
}

impl HostTypeDescriptor {
    pub(in crate::host) fn of<Type: HostType>() -> Self {
        <Type as private::Abi>::descriptor()
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
            Self::Function { arguments, return_ } | Self::OpaqueFunction { arguments, return_ } => {
                for argument in arguments {
                    argument.collect_external_schemas(output, visited);
                }
                return_.collect_external_schemas(output, visited);
            }
            Self::Custom { schema, arguments } => {
                for constructor in schema.constructors() {
                    for field in constructor.fields() {
                        field.type_().collect_external_schemas(output, visited);
                    }
                }
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

    #[cfg(test)]
    pub(crate) fn schema_type(&self) -> HostSchemaType {
        match self {
            Self::Parameter(index) => HostSchemaType::Parameter(*index),
            Self::Int => HostSchemaType::Int,
            Self::Float => HostSchemaType::Float,
            Self::String => HostSchemaType::String,
            Self::BitArray => HostSchemaType::BitArray,
            Self::UtfCodepoint => HostSchemaType::UtfCodepoint,
            Self::Bool => HostSchemaType::Bool,
            Self::Nil => HostSchemaType::Nil,
            Self::List(item) => HostSchemaType::List(Box::new(item.schema_type())),
            Self::Tuple(elements) => HostSchemaType::Tuple(
                elements
                    .iter()
                    .map(Self::schema_type)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            Self::Function { arguments, return_ } | Self::OpaqueFunction { arguments, return_ } => {
                HostSchemaType::Function {
                    arguments: arguments
                        .iter()
                        .map(Self::schema_type)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                    return_: Box::new(return_.schema_type()),
                }
            }
            Self::Custom { schema, arguments } => HostSchemaType::Custom {
                package: schema.package().clone(),
                module: schema.module().clone(),
                name: schema.name().clone(),
                arguments: arguments
                    .iter()
                    .map(Self::schema_type)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
            Self::External { schema, arguments } => HostSchemaType::External {
                schema: schema.clone(),
                arguments: arguments
                    .iter()
                    .map(Self::schema_type)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
        }
    }

    pub(crate) fn value_shape(&self) -> crate::plan::ValueShape {
        match self {
            Self::Parameter(index) => {
                crate::plan::ValueShape::Parameter(crate::plan::TypeParameterId(*index))
            }
            Self::Int => crate::plan::ValueShape::Int,
            Self::Float => crate::plan::ValueShape::Float,
            Self::String => crate::plan::ValueShape::String,
            Self::BitArray => crate::plan::ValueShape::BitArray,
            Self::UtfCodepoint => crate::plan::ValueShape::UtfCodepoint,
            Self::Bool => crate::plan::ValueShape::Bool,
            Self::Nil => crate::plan::ValueShape::Nil,
            Self::List(item) => crate::plan::ValueShape::List(Box::new(item.value_shape())),
            Self::Tuple(elements) => crate::plan::ValueShape::Tuple(
                elements
                    .iter()
                    .map(Self::value_shape)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            Self::Function { arguments, return_ } | Self::OpaqueFunction { arguments, return_ } => {
                crate::plan::ValueShape::Function(Box::new(crate::plan::FunctionShape::new(
                    arguments.iter().map(Self::value_shape).collect(),
                    return_.value_shape(),
                )))
            }
            Self::Custom { schema, arguments } => crate::plan::ValueShape::Custom(
                crate::plan::CustomValueShape::any(crate::plan::CustomType::new(
                    crate::plan::CustomTypeName::new(
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    arguments.iter().map(Self::value_type).collect(),
                )),
            ),
            Self::External { schema, arguments } => {
                crate::plan::ValueShape::External(crate::plan::ExternalValueShape::new(
                    crate::plan::ExternalTypeName::new(
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    arguments.iter().map(Self::value_shape).collect(),
                ))
            }
        }
    }

    pub(crate) fn value_type(&self) -> crate::plan::ValueType {
        self.value_shape().value_type()
    }

    pub(crate) fn resolve(
        &self,
        type_arguments: &[crate::plan::ValueType],
    ) -> Option<crate::plan::ValueType> {
        use crate::plan::ValueType;

        match self {
            Self::Parameter(index) => type_arguments.get(*index).cloned(),
            Self::Int => Some(ValueType::Int),
            Self::Float => Some(ValueType::Float),
            Self::String => Some(ValueType::String),
            Self::BitArray => Some(ValueType::BitArray),
            Self::UtfCodepoint => Some(ValueType::UtfCodepoint),
            Self::Bool => Some(ValueType::Bool),
            Self::Nil => Some(ValueType::Nil),
            Self::List(item) => Some(ValueType::List(Box::new(item.resolve(type_arguments)?))),
            Self::Tuple(elements) => Some(ValueType::Tuple(
                elements
                    .iter()
                    .map(|element| element.resolve(type_arguments))
                    .collect::<Option<Vec<_>>>()?,
            )),
            Self::Function { arguments, return_ } | Self::OpaqueFunction { arguments, return_ } => {
                Some(ValueType::Function(Box::new(
                    crate::plan::FunctionType::new(
                        arguments
                            .iter()
                            .map(|argument| argument.resolve(type_arguments))
                            .collect::<Option<Vec<_>>>()?,
                        return_.resolve(type_arguments)?,
                    ),
                )))
            }
            Self::Custom { schema, arguments } => {
                Some(ValueType::Custom(crate::plan::CustomType::new(
                    crate::plan::CustomTypeName::new(
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    arguments
                        .iter()
                        .map(|argument| argument.resolve(type_arguments))
                        .collect::<Option<Vec<_>>>()?,
                )))
            }
            Self::External { schema, arguments } => {
                Some(ValueType::External(crate::plan::ExternalType::new(
                    crate::plan::ExternalTypeName::new(
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    arguments
                        .iter()
                        .map(|argument| argument.resolve(type_arguments))
                        .collect::<Option<Vec<_>>>()?,
                )))
            }
        }
    }

    pub(crate) fn resolve_sealed(
        &self,
        type_arguments: &[crate::plan::ValueType],
    ) -> crate::plan::ValueType {
        use crate::plan::ValueType;

        match self {
            Self::Parameter(index) => type_arguments[*index].clone(),
            Self::Int => ValueType::Int,
            Self::Float => ValueType::Float,
            Self::String => ValueType::String,
            Self::BitArray => ValueType::BitArray,
            Self::UtfCodepoint => ValueType::UtfCodepoint,
            Self::Bool => ValueType::Bool,
            Self::Nil => ValueType::Nil,
            Self::List(item) => ValueType::List(Box::new(item.resolve_sealed(type_arguments))),
            Self::Tuple(elements) => ValueType::Tuple(
                elements
                    .iter()
                    .map(|element| element.resolve_sealed(type_arguments))
                    .collect(),
            ),
            Self::Function { arguments, return_ } | Self::OpaqueFunction { arguments, return_ } => {
                ValueType::Function(Box::new(crate::plan::FunctionType::new(
                    arguments
                        .iter()
                        .map(|argument| argument.resolve_sealed(type_arguments))
                        .collect(),
                    return_.resolve_sealed(type_arguments),
                )))
            }
            Self::Custom { schema, arguments } => ValueType::Custom(crate::plan::CustomType::new(
                crate::plan::CustomTypeName::new(
                    schema.package().clone(),
                    schema.module().clone(),
                    schema.name().clone(),
                ),
                arguments
                    .iter()
                    .map(|argument| argument.resolve_sealed(type_arguments))
                    .collect(),
            )),
            Self::External { schema, arguments } => {
                ValueType::External(crate::plan::ExternalType::new(
                    crate::plan::ExternalTypeName::new(
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    arguments
                        .iter()
                        .map(|argument| argument.resolve_sealed(type_arguments))
                        .collect(),
                ))
            }
        }
    }

    pub(crate) fn collect_type_parameters(&self, output: &mut BTreeSet<usize>) {
        match self {
            Self::Parameter(index) => {
                output.insert(*index);
            }
            Self::List(item) => item.collect_type_parameters(output),
            Self::Tuple(elements) => {
                for element in elements {
                    element.collect_type_parameters(output);
                }
            }
            Self::Function { arguments, return_ } | Self::OpaqueFunction { arguments, return_ } => {
                for argument in arguments {
                    argument.collect_type_parameters(output);
                }
                return_.collect_type_parameters(output);
            }
            Self::Custom { arguments, .. } | Self::External { arguments, .. } => {
                for argument in arguments {
                    argument.collect_type_parameters(output);
                }
            }
            Self::Int
            | Self::Float
            | Self::String
            | Self::BitArray
            | Self::UtfCodepoint
            | Self::Bool
            | Self::Nil => {}
        }
    }
}

mod private {
    pub(crate) trait Sealed {}

    pub(crate) trait Abi {
        fn descriptor() -> super::HostTypeDescriptor;
        fn schema_type() -> super::HostSchemaType;
        fn collect_custom_schemas(
            _output: &mut Vec<super::HostCustomTypeSchema>,
            _visited: &mut std::collections::HashSet<super::HostCustomIdentity>,
        ) {
        }
        fn into_scoped(value: <Self as super::HostType>::Value<'_>) -> super::HostScopedValue
        where
            Self: super::HostType;
        fn from_token<'call, Profile: crate::host::HostProfile>(
            runtime: &dyn crate::host::HostCallRuntime<Profile>,
            token: crate::host::HostValueToken,
        ) -> <Self as super::HostType>::Value<'call>
        where
            Self: super::HostType;
    }

    pub(crate) trait Sequence {
        fn descriptors() -> Vec<super::HostTypeDescriptor>;
        fn schema_types() -> Vec<super::HostSchemaType>;
        fn collect_custom_schemas(
            output: &mut Vec<super::HostCustomTypeSchema>,
            visited: &mut std::collections::HashSet<super::HostCustomIdentity>,
        );
        fn into_scoped_values(
            values: <Self as super::HostTypeSequence>::Values<'_>,
            output: &mut Vec<super::HostScopedValue>,
        ) where
            Self: super::HostTypeSequence;
        fn from_tokens<'call, Profile: crate::host::HostProfile>(
            runtime: &dyn crate::host::HostCallRuntime<Profile>,
            tokens: &[crate::host::HostValueToken],
            index: &mut usize,
        ) -> <Self as super::HostTypeSequence>::Values<'call>
        where
            Self: super::HostTypeSequence;
    }

    pub(crate) trait CustomConstructors {
        fn schemas() -> Vec<super::HostCustomConstructorSchema>;
        fn collect_custom_schemas(
            output: &mut Vec<super::HostCustomTypeSchema>,
            visited: &mut std::collections::HashSet<super::HostCustomIdentity>,
        );
    }

    pub(crate) trait CustomFields {
        fn schemas() -> Vec<super::HostCustomFieldSchema>;
        fn collect_custom_schemas(
            output: &mut Vec<super::HostCustomTypeSchema>,
            visited: &mut std::collections::HashSet<super::HostCustomIdentity>,
        );
    }

    pub(crate) trait ConstructorAt<Index, Definition> {
        fn index() -> usize;
    }

    pub(crate) trait CustomConstructor {
        fn index() -> usize;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostAbiType, HostAbiTypeSequence, HostCustomConstructor, HostCustomConstructorAt,
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomConstructorSchema, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
        HostCustomFieldSchema, HostCustomIndex0, HostCustomSchema, HostCustomType,
        HostCustomTypeSchema, HostListType, HostSchemaType, HostTupleType, HostTypeDescriptor,
        HostTypeList, HostTypeListEnd, HostTypeParameter, from_token, from_tokens,
    };
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostCustom, HostCustomToken, HostExternalTypeSchema, HostList, HostListToken,
        HostScopedValue, HostTuple, HostTupleToken, HostValue, HostValueFamily, HostValueToken,
    };
    use crate::runtime::BitArrayValue;
    use ecow::EcoString;
    use num_bigint::BigInt;

    struct MarkerSchema;

    struct MarkerConstructor;

    impl HostCustomConstructorDefinition for MarkerConstructor {
        const NAME: &'static str = "Marker";

        type Fields = HostCustomFieldListEnd;
    }

    impl HostCustomSchema for MarkerSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/marker";
        const NAME: &'static str = "Marker";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<MarkerConstructor, HostCustomConstructorListEnd>;
    }

    struct RecursiveSchema;

    struct ConflictingRecursiveSchema;

    struct RecursiveNextField;

    impl HostCustomField for RecursiveNextField {
        const LABEL: Option<&'static str> = Some("next");

        type Type = HostListType<HostCustomType<RecursiveSchema>>;
    }

    struct RecursiveConstructor;

    impl HostCustomConstructorDefinition for RecursiveConstructor {
        const NAME: &'static str = "Node";

        type Fields = HostCustomFieldList<RecursiveNextField, HostCustomFieldListEnd>;
    }

    impl HostCustomSchema for RecursiveSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/tree";
        const NAME: &'static str = "Tree";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<RecursiveConstructor, HostCustomConstructorListEnd>;
    }

    struct ConflictingRecursiveConstructor;

    impl HostCustomConstructorDefinition for ConflictingRecursiveConstructor {
        const NAME: &'static str = "Leaf";

        type Fields = HostCustomFieldListEnd;
    }

    impl HostCustomSchema for ConflictingRecursiveSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/tree";
        const NAME: &'static str = "Tree";
        const PARAMETER_COUNT: usize = 0;

        type Constructors = HostCustomConstructorList<
            ConflictingRecursiveConstructor,
            HostCustomConstructorListEnd,
        >;
    }

    #[test]
    fn custom_schema_owns_nominal_identity_and_complete_constructor_fields() {
        let schema = HostCustomTypeSchema::new(
            "domain",
            "domain/box",
            "Boxed",
            1,
            [
                HostCustomConstructorSchema::new(
                    "Boxed",
                    [HostCustomFieldSchema::new(
                        Some("value"),
                        HostSchemaType::parameter(0),
                    )],
                ),
                HostCustomConstructorSchema::new("Empty", Vec::<HostCustomFieldSchema>::new()),
            ],
        );

        assert_eq!(schema.package(), "domain");
        assert_eq!(schema.module(), "domain/box");
        assert_eq!(schema.name(), "Boxed");
        assert_eq!(schema.parameter_count(), 1);
        assert_eq!(schema.constructors()[0].name(), "Boxed");
        assert_eq!(
            schema.constructors()[0].fields()[0].label(),
            Some(&"value".into())
        );
        assert_eq!(
            schema.constructors()[0].fields()[0].type_(),
            &HostSchemaType::Parameter(0),
        );
        assert_eq!(schema.constructors()[1].fields(), []);
    }

    #[test]
    fn custom_schema_collection_is_recursive_and_cycle_safe() {
        type Recursive = HostCustomType<RecursiveSchema>;

        let mut schemas = Vec::new();
        let mut visited = std::collections::HashSet::new();
        <Recursive as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);

        assert_eq!(
            schemas,
            [HostCustomTypeSchema::new(
                "domain",
                "domain/tree",
                "Tree",
                0,
                [HostCustomConstructorSchema::new(
                    "Node",
                    [HostCustomFieldSchema::new(
                        Some("next"),
                        HostSchemaType::list(HostSchemaType::custom(
                            "domain",
                            "domain/tree",
                            "Tree",
                            Vec::<HostSchemaType>::new(),
                        )),
                    )],
                )],
            )],
        );
    }

    #[test]
    fn custom_schema_collection_retains_conflicting_nominal_definitions() {
        type Types = HostTypeList<
            HostCustomType<RecursiveSchema>,
            HostTypeList<HostCustomType<ConflictingRecursiveSchema>, HostTypeListEnd>,
        >;

        let mut schemas = Vec::new();
        let mut visited = std::collections::HashSet::new();
        <Types as HostAbiTypeSequence>::collect_custom_schemas(&mut schemas, &mut visited);

        assert_eq!(
            schemas,
            [
                HostCustomTypeSchema::of::<RecursiveSchema>(),
                HostCustomTypeSchema::of::<ConflictingRecursiveSchema>(),
            ],
        );
    }

    #[test]
    fn recursive_descriptor_tracks_type_parameters_and_schema_types() {
        let schema = HostCustomTypeSchema::new("domain", "domain/box", "Boxed", 1, Vec::new());
        let descriptor = HostTypeDescriptor::Tuple(
            vec![
                HostTypeDescriptor::Parameter(2),
                HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Custom {
                    schema: schema.clone(),
                    arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
                })),
            ]
            .into_boxed_slice(),
        );
        let mut type_parameters = std::collections::BTreeSet::new();
        descriptor.collect_type_parameters(&mut type_parameters);

        assert_eq!(type_parameters, std::collections::BTreeSet::from([0, 2]));
        assert_eq!(
            descriptor.schema_type(),
            HostSchemaType::tuple([
                HostSchemaType::parameter(2),
                HostSchemaType::list(HostSchemaType::custom(
                    "domain",
                    "domain/box",
                    "Boxed",
                    [HostSchemaType::parameter(0)],
                )),
            ]),
        );
    }

    #[test]
    fn descriptor_resolves_every_shape_through_concrete_type_arguments() {
        use crate::plan::{
            CustomType, CustomTypeName, ExternalType, ExternalTypeName, FunctionType, ValueType,
        };

        let arguments = [ValueType::String, ValueType::Bool];
        let custom_schema =
            HostCustomTypeSchema::new("domain", "domain/box", "Boxed", 1, Vec::new());
        let external_schema =
            HostExternalTypeSchema::new("domain", "domain/resource", "Resource", 1);
        let descriptor = HostTypeDescriptor::Tuple(
            vec![
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Float,
                HostTypeDescriptor::String,
                HostTypeDescriptor::BitArray,
                HostTypeDescriptor::UtfCodepoint,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Nil,
                HostTypeDescriptor::Parameter(0),
                HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Parameter(1))),
                HostTypeDescriptor::Function {
                    arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
                    return_: Box::new(HostTypeDescriptor::Parameter(1)),
                },
                HostTypeDescriptor::OpaqueFunction {
                    arguments: vec![HostTypeDescriptor::Parameter(1)].into_boxed_slice(),
                    return_: Box::new(HostTypeDescriptor::Parameter(0)),
                },
                HostTypeDescriptor::Custom {
                    schema: custom_schema,
                    arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
                },
                HostTypeDescriptor::External {
                    schema: external_schema,
                    arguments: vec![HostTypeDescriptor::Parameter(1)].into_boxed_slice(),
                },
            ]
            .into_boxed_slice(),
        );

        let expected = ValueType::Tuple(vec![
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::String,
            ValueType::List(Box::new(ValueType::Bool)),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::String],
                ValueType::Bool,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Bool],
                ValueType::String,
            ))),
            ValueType::Custom(CustomType::new(
                CustomTypeName::new("domain".into(), "domain/box".into(), "Boxed".into()),
                vec![ValueType::String],
            )),
            ValueType::External(ExternalType::new(
                ExternalTypeName::new("domain".into(), "domain/resource".into(), "Resource".into()),
                vec![ValueType::Bool],
            )),
        ]);
        assert_eq!(descriptor.resolve(&arguments), Some(expected.clone()));
        assert_eq!(descriptor.resolve_sealed(&arguments), expected);
        assert_eq!(HostTypeDescriptor::Parameter(2).resolve(&arguments), None);

        let missing = HostTypeDescriptor::Parameter(2);
        assert_eq!(
            HostTypeDescriptor::List(Box::new(missing.clone())).resolve(&arguments),
            None,
        );
        assert_eq!(
            HostTypeDescriptor::Tuple(vec![missing.clone()].into_boxed_slice()).resolve(&arguments),
            None,
        );
        assert_eq!(
            HostTypeDescriptor::Function {
                arguments: vec![missing.clone()].into_boxed_slice(),
                return_: Box::new(HostTypeDescriptor::Int),
            }
            .resolve(&arguments),
            None,
        );
        assert_eq!(
            HostTypeDescriptor::Function {
                arguments: Box::new([]),
                return_: Box::new(missing.clone()),
            }
            .resolve(&arguments),
            None,
        );
        assert_eq!(
            HostTypeDescriptor::Custom {
                schema: HostCustomTypeSchema::new("domain", "domain/box", "Boxed", 1, Vec::new(),),
                arguments: vec![missing.clone()].into_boxed_slice(),
            }
            .resolve(&arguments),
            None,
        );
        assert_eq!(
            HostTypeDescriptor::External {
                schema: HostExternalTypeSchema::new("domain", "domain/resource", "Resource", 1,),
                arguments: vec![missing].into_boxed_slice(),
            }
            .resolve(&arguments),
            None,
        );
    }

    #[test]
    fn schema_types_include_function_fields_without_runtime_metadata() {
        let descriptors = [
            (HostTypeDescriptor::Int, HostSchemaType::Int),
            (HostTypeDescriptor::Float, HostSchemaType::Float),
            (HostTypeDescriptor::String, HostSchemaType::String),
            (HostTypeDescriptor::BitArray, HostSchemaType::BitArray),
            (
                HostTypeDescriptor::UtfCodepoint,
                HostSchemaType::UtfCodepoint,
            ),
            (HostTypeDescriptor::Bool, HostSchemaType::Bool),
            (HostTypeDescriptor::Nil, HostSchemaType::Nil),
        ];

        for (descriptor, schema) in descriptors {
            assert_eq!(descriptor.schema_type(), schema);
        }
        assert_eq!(
            HostTypeDescriptor::Function {
                arguments: vec![HostTypeDescriptor::Int, HostTypeDescriptor::Bool]
                    .into_boxed_slice(),
                return_: Box::new(HostTypeDescriptor::String),
            }
            .schema_type(),
            HostSchemaType::Function {
                arguments: vec![HostSchemaType::Int, HostSchemaType::Bool].into_boxed_slice(),
                return_: Box::new(HostSchemaType::String),
            },
        );
        assert_eq!(
            HostTypeDescriptor::OpaqueFunction {
                arguments: vec![HostTypeDescriptor::String].into_boxed_slice(),
                return_: Box::new(HostTypeDescriptor::Nil),
            }
            .schema_type(),
            HostSchemaType::function([HostSchemaType::String], HostSchemaType::Nil),
        );
        let external = HostExternalTypeSchema::new("domain", "domain/resource", "Resource", 1);
        assert_eq!(
            HostTypeDescriptor::External {
                schema: external.clone(),
                arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
            }
            .schema_type(),
            HostSchemaType::External {
                schema: external,
                arguments: vec![HostSchemaType::parameter(0)].into_boxed_slice(),
            },
        );
    }

    #[test]
    fn abi_type_sequence_preserves_scalar_and_recursive_source_schema_order() {
        type Parameter = HostTypeParameter<0>;
        type List = HostListType<Parameter>;
        type TupleElements = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;
        type Tuple = HostTupleType<TupleElements>;
        type Custom = HostCustomType<MarkerSchema>;
        type Types = HostTypeList<
            BigInt,
            HostTypeList<
                f64,
                HostTypeList<
                    EcoString,
                    HostTypeList<
                        BitArrayValue,
                        HostTypeList<
                            char,
                            HostTypeList<
                                bool,
                                HostTypeList<
                                    (),
                                    HostTypeList<
                                        Parameter,
                                        HostTypeList<
                                            List,
                                            HostTypeList<
                                                Tuple,
                                                HostTypeList<Custom, HostTypeListEnd>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >;

        assert_eq!(
            <Types as HostAbiTypeSequence>::schema_types(),
            [
                HostSchemaType::Int,
                HostSchemaType::Float,
                HostSchemaType::String,
                HostSchemaType::BitArray,
                HostSchemaType::UtfCodepoint,
                HostSchemaType::Bool,
                HostSchemaType::Nil,
                HostSchemaType::Parameter(0),
                HostSchemaType::list(HostSchemaType::Parameter(0)),
                HostSchemaType::tuple([HostSchemaType::Int, HostSchemaType::Bool]),
                HostSchemaType::custom(
                    "domain",
                    "domain/marker",
                    "Marker",
                    Vec::<HostSchemaType>::new(),
                ),
            ],
        );
    }

    #[test]
    fn custom_field_sequence_preserves_layout_and_nested_custom_schemas() {
        type Recursive = HostCustomType<RecursiveSchema>;
        type Constructor =
            HostCustomConstructorAt<Recursive, HostCustomIndex0, RecursiveConstructor>;
        type Fields = <Constructor as HostCustomConstructor>::Fields;

        assert_eq!(
            <Fields as HostAbiTypeSequence>::descriptors(),
            [HostTypeDescriptor::List(Box::new(
                <Recursive as HostAbiType>::descriptor(),
            ))],
        );
        assert_eq!(
            <Fields as HostAbiTypeSequence>::schema_types(),
            [HostSchemaType::list(HostSchemaType::custom(
                "domain",
                "domain/tree",
                "Tree",
                Vec::<HostSchemaType>::new(),
            ))],
        );

        let mut schemas = Vec::new();
        let mut visited = std::collections::HashSet::new();
        <Fields as HostAbiTypeSequence>::collect_custom_schemas(&mut schemas, &mut visited);

        assert_eq!(schemas, [HostCustomTypeSchema::of::<RecursiveSchema>()]);
    }

    #[test]
    fn abi_types_preserve_each_typed_value_and_call_scoped_token() {
        type Parameter = HostTypeParameter<0>;
        type List = HostListType<BigInt>;
        type Elements = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;
        type Tuple = HostTupleType<Elements>;
        type Custom = HostCustomType<MarkerSchema>;

        assert_eq!(
            <BigInt as HostAbiType>::descriptor(),
            HostTypeDescriptor::Int
        );
        assert_eq!(
            <f64 as HostAbiType>::descriptor(),
            HostTypeDescriptor::Float
        );
        assert_eq!(
            <EcoString as HostAbiType>::descriptor(),
            HostTypeDescriptor::String,
        );
        assert_eq!(
            <BitArrayValue as HostAbiType>::descriptor(),
            HostTypeDescriptor::BitArray,
        );
        assert_eq!(
            <char as HostAbiType>::descriptor(),
            HostTypeDescriptor::UtfCodepoint,
        );
        assert_eq!(
            <bool as HostAbiType>::descriptor(),
            HostTypeDescriptor::Bool
        );
        assert_eq!(<() as HostAbiType>::descriptor(), HostTypeDescriptor::Nil);
        assert_eq!(
            <Parameter as HostAbiType>::descriptor(),
            HostTypeDescriptor::Parameter(0),
        );
        assert_eq!(
            <List as HostAbiType>::descriptor(),
            HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int)),
        );
        assert_eq!(
            <Tuple as HostAbiType>::descriptor(),
            HostTypeDescriptor::Tuple(
                vec![HostTypeDescriptor::Int, HostTypeDescriptor::Bool].into_boxed_slice(),
            ),
        );
        assert_eq!(
            <Custom as HostAbiType>::descriptor(),
            HostTypeDescriptor::Custom {
                schema: HostCustomTypeSchema::of::<MarkerSchema>(),
                arguments: Box::new([]),
            },
        );

        assert_eq!(
            <BigInt as HostAbiType>::into_scoped(1.into()),
            HostScopedValue::Int(BigInt::from(1)),
        );
        assert_eq!(
            <f64 as HostAbiType>::into_scoped(1.5),
            HostScopedValue::Float(1.5),
        );
        assert_eq!(
            <EcoString as HostAbiType>::into_scoped("text".into()),
            HostScopedValue::String("text".into()),
        );
        assert_eq!(
            <BitArrayValue as HostAbiType>::into_scoped(BitArrayValue::from_bytes(vec![1])),
            HostScopedValue::BitArray(BitArrayValue::from_bytes(vec![1])),
        );
        assert_eq!(
            <char as HostAbiType>::into_scoped('A'),
            HostScopedValue::UtfCodepoint('A'),
        );
        assert_eq!(
            <bool as HostAbiType>::into_scoped(true),
            HostScopedValue::Bool(true),
        );
        assert_eq!(<() as HostAbiType>::into_scoped(()), HostScopedValue::Nil,);

        let parameter_token = HostValueToken {
            family: HostValueFamily::Bool,
            index: 1,
        };
        assert_eq!(
            <Parameter as HostAbiType>::into_scoped(HostValue::new(parameter_token)),
            HostScopedValue::Value(parameter_token),
        );
        assert_eq!(
            <List as HostAbiType>::into_scoped(HostList::new(HostListToken::Stored(2))),
            HostScopedValue::List(HostListToken::Stored(2)),
        );
        assert_eq!(
            <Tuple as HostAbiType>::into_scoped(HostTuple::new(HostTupleToken(3))),
            HostScopedValue::Tuple(HostTupleToken(3)),
        );
        assert_eq!(
            <Custom as HostAbiType>::into_scoped(HostCustom::new(HostCustomToken(4))),
            HostScopedValue::Custom(HostCustomToken(4)),
        );

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let token = HostValueToken {
            family: HostValueFamily::Int,
            index: 0,
        };

        assert_eq!(
            from_token::<BigInt, TestHostProfile>(&runtime, token),
            BigInt::from(0),
        );
        assert_eq!(from_token::<f64, TestHostProfile>(&runtime, token), 0.0,);
        assert_eq!(
            from_token::<EcoString, TestHostProfile>(&runtime, token),
            "",
        );
        assert_eq!(
            from_token::<BitArrayValue, TestHostProfile>(&runtime, token),
            BitArrayValue::from_bytes(Vec::new()),
        );
        assert_eq!(from_token::<char, TestHostProfile>(&runtime, token), '\0',);
        assert!(!from_token::<bool, TestHostProfile>(&runtime, token));
        from_token::<(), TestHostProfile>(&runtime, token);
        assert_eq!(
            from_token::<Parameter, TestHostProfile>(&runtime, parameter_token).token,
            parameter_token,
        );
        assert_eq!(
            from_token::<List, TestHostProfile>(&runtime, token).token,
            HostListToken::Stored(0),
        );
        assert_eq!(
            from_token::<Tuple, TestHostProfile>(&runtime, token).token,
            HostTupleToken(0),
        );
        assert_eq!(
            from_token::<Custom, TestHostProfile>(&runtime, token).token,
            HostCustomToken(0),
        );

        let values = from_tokens::<Elements, TestHostProfile>(&runtime, &[token, token]);
        assert_eq!(values, (BigInt::from(0), (false, ())));
        let mut scoped = Vec::new();
        <Elements as HostAbiTypeSequence>::into_scoped_values(
            (BigInt::from(2), (true, ())),
            &mut scoped,
        );
        assert!(matches!(scoped[0], HostScopedValue::Int(ref value) if value == &BigInt::from(2)));
        assert!(matches!(scoped[1], HostScopedValue::Bool(true)));
    }
}
