use super::{HostCustomIdentity, HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor};
use std::collections::HashMap;

impl HostTypeDescriptor {
    pub(crate) fn from_schema(
        type_: &HostSchemaType,
        arguments: &[Self],
        schemas: &HashMap<HostCustomIdentity, &HostCustomTypeSchema>,
    ) -> Self {
        match type_ {
            HostSchemaType::Parameter(index) => arguments[*index].clone(),
            HostSchemaType::Int => Self::Int,
            HostSchemaType::Float => Self::Float,
            HostSchemaType::String => Self::String,
            HostSchemaType::BitArray => Self::BitArray,
            HostSchemaType::UtfCodepoint => Self::UtfCodepoint,
            HostSchemaType::Bool => Self::Bool,
            HostSchemaType::Nil => Self::Nil,
            HostSchemaType::List(item) => {
                Self::List(Box::new(Self::from_schema(item, arguments, schemas)))
            }
            HostSchemaType::Tuple(elements) => Self::Tuple(
                elements
                    .iter()
                    .map(|item| Self::from_schema(item, arguments, schemas))
                    .collect(),
            ),
            HostSchemaType::Function {
                arguments: inputs,
                return_,
            } => Self::Function {
                arguments: inputs
                    .iter()
                    .map(|item| Self::from_schema(item, arguments, schemas))
                    .collect(),
                return_: Box::new(Self::from_schema(return_, arguments, schemas)),
            },
            HostSchemaType::Custom {
                package,
                module,
                name,
                arguments: inputs,
            } => Self::Custom {
                schema: schemas[&(package.clone(), module.clone(), name.clone())].clone(),
                arguments: inputs
                    .iter()
                    .map(|item| Self::from_schema(item, arguments, schemas))
                    .collect(),
            },
            HostSchemaType::External {
                schema,
                arguments: inputs,
            } => Self::External {
                schema: schema.clone(),
                arguments: inputs
                    .iter()
                    .map(|item| Self::from_schema(item, arguments, schemas))
                    .collect(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HashMap, HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor};
    use crate::host::{HostCustomConstructorSchema, HostCustomFieldSchema, HostExternalTypeSchema};

    #[test]
    fn resolves_all_schema_shapes_and_preserves_opaque_argument_capabilities() {
        let custom = HostCustomTypeSchema::new(
            "domain",
            "domain/box",
            "Box",
            1,
            [HostCustomConstructorSchema::new(
                "Box",
                [HostCustomFieldSchema::new(
                    None::<&str>,
                    HostSchemaType::Parameter(0),
                )],
            )],
        );
        let external = HostExternalTypeSchema::new("domain", "domain/resource", "Resource", 1);
        let schemas = HashMap::from([(
            (
                custom.package().clone(),
                custom.module().clone(),
                custom.name().clone(),
            ),
            &custom,
        )]);
        let opaque = HostTypeDescriptor::OpaqueFunction {
            arguments: Box::new([]),
            return_: Box::new(HostTypeDescriptor::String),
        };
        for (source, expected) in [
            (HostSchemaType::Parameter(0), opaque.clone()),
            (HostSchemaType::Int, HostTypeDescriptor::Int),
            (HostSchemaType::Float, HostTypeDescriptor::Float),
            (HostSchemaType::String, HostTypeDescriptor::String),
            (HostSchemaType::BitArray, HostTypeDescriptor::BitArray),
            (
                HostSchemaType::UtfCodepoint,
                HostTypeDescriptor::UtfCodepoint,
            ),
            (HostSchemaType::Bool, HostTypeDescriptor::Bool),
            (HostSchemaType::Nil, HostTypeDescriptor::Nil),
            (
                HostSchemaType::list(HostSchemaType::Parameter(0)),
                HostTypeDescriptor::List(Box::new(opaque.clone())),
            ),
            (
                HostSchemaType::tuple([HostSchemaType::Bool, HostSchemaType::Parameter(0)]),
                HostTypeDescriptor::Tuple(Box::new([HostTypeDescriptor::Bool, opaque.clone()])),
            ),
            (
                HostSchemaType::function([HostSchemaType::Parameter(0)], HostSchemaType::Int),
                HostTypeDescriptor::Function {
                    arguments: Box::new([opaque.clone()]),
                    return_: Box::new(HostTypeDescriptor::Int),
                },
            ),
            (
                HostSchemaType::custom(
                    "domain",
                    "domain/box",
                    "Box",
                    [HostSchemaType::Parameter(0)],
                ),
                HostTypeDescriptor::Custom {
                    schema: custom.clone(),
                    arguments: Box::new([opaque.clone()]),
                },
            ),
            (
                HostSchemaType::External {
                    schema: external.clone(),
                    arguments: Box::new([HostSchemaType::list(HostSchemaType::Int)]),
                },
                HostTypeDescriptor::External {
                    schema: external,
                    arguments: Box::new([HostTypeDescriptor::List(Box::new(
                        HostTypeDescriptor::Int,
                    ))]),
                },
            ),
        ] {
            assert_eq!(
                HostTypeDescriptor::from_schema(&source, std::slice::from_ref(&opaque), &schemas),
                expected
            );
        }
    }
}
