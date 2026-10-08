use super::NativeError;
use crate::host::{HostCustomAccess, HostCustomTypeSchema, HostSchemaType};
use crate::plan::CustomTypeName;
use crate::plan::execution::type_::TypeMetadata;
use crate::plan::execution::type_::custom::CustomDefinition;
use std::collections::HashMap;

pub(super) type CustomGrants = HashMap<CustomTypeName, HostCustomTypeSchema>;

/// Revalidates producer permissions before the artifact gets a usable owner.
pub(super) fn admit(
    definitions: &[CustomDefinition],
    grants: &CustomGrants,
) -> Result<(), NativeError> {
    for definition in definitions {
        let name = CustomTypeName::new(
            definition.package.as_str().into(),
            definition.module.as_str().into(),
            definition.name.as_str().into(),
        );
        let registered = grants.get(&name);
        if registered.map(HostCustomTypeSchema::access) != definition.native_access
            || registered.is_some_and(|schema| !matches(definition, schema))
        {
            return Err(NativeError::CustomGrant {
                custom_type: Box::new(name),
            });
        }
    }
    Ok(())
}

fn matches(definition: &CustomDefinition, schema: &HostCustomTypeSchema) -> bool {
    if definition.parameters != schema.parameter_count()
        || definition.retention_lifetime != schema.lifetime()
    {
        return false;
    }
    if schema.access() == HostCustomAccess::Retained {
        return true;
    }
    definition.constructors.len() == schema.constructors().len()
        && definition
            .constructors
            .iter()
            .zip(schema.constructors())
            .all(|(stored, original)| {
                stored.name.as_str() == original.name().as_str()
                    && stored.fields.len() == original.fields().len()
                    && stored
                        .fields
                        .iter()
                        .zip(original.fields())
                        .all(|(stored, original)| {
                            stored.label.as_deref() == original.label().map(|label| label.as_str())
                                && field(&stored.type_, original.type_())
                        })
            })
}

fn field(stored: &TypeMetadata, original: &HostSchemaType) -> bool {
    use HostSchemaType as H;
    use TypeMetadata as T;
    match (stored, original) {
        (T::Parameter(left), H::Parameter(right)) => left.0 == *right,
        (T::Int, H::Int)
        | (T::Float, H::Float)
        | (T::String, H::String)
        | (T::BitArray, H::BitArray)
        | (T::UtfCodepoint, H::UtfCodepoint)
        | (T::Bool, H::Bool)
        | (T::Nil, H::Nil) => true,
        (T::List(left), H::List(right)) => field(left, right),
        (T::Tuple(left), H::Tuple(right)) => fields(left, right),
        (T::Function(left), H::Function { arguments, return_ }) => {
            fields(&left.arguments, arguments) && field(&left.return_, return_)
        }
        (
            T::Custom(left),
            H::Custom {
                package,
                module,
                name,
                arguments,
            },
        ) => {
            left.package.as_str() == package.as_str()
                && left.module.as_str() == module.as_str()
                && left.name.as_str() == name.as_str()
                && fields(&left.arguments, arguments)
        }
        (T::External(left), H::External { schema, arguments }) => {
            left.package.as_str() == schema.package().as_str()
                && left.module.as_str() == schema.module().as_str()
                && left.name.as_str() == schema.name().as_str()
                && left.arguments.len() == schema.parameter_count()
                && fields(&left.arguments, arguments)
        }
        _ => false,
    }
}

fn fields(stored: &[TypeMetadata], original: &[HostSchemaType]) -> bool {
    stored.len() == original.len()
        && stored
            .iter()
            .zip(original)
            .all(|(left, right)| field(left, right))
}

#[cfg(test)]
mod tests {
    use super::{CustomGrants, NativeError, admit, field};
    use crate::host::{
        HostCustomAccess, HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema,
        HostExternalTypeSchema, HostRetainedCustomSchema, HostSchemaType, HostValueLifetime,
    };
    use crate::plan::execution::storage::Node;
    use crate::plan::execution::type_::{NominalTypeMetadata, TypeMetadata};
    use crate::plan::{CustomTypeName, TypeParameterId};

    struct Retained;
    impl HostRetainedCustomSchema for Retained {
        const PACKAGE: &'static str = "geam";
        const MODULE: &'static str = "example";
        const NAME: &'static str = "Envelope";
        const PARAMETER_COUNT: usize = 1;
    }

    #[test]
    fn producer_grants_keep_nominal_retention_distinct_from_full_representation_sharing() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Box(a) { Box(a) }
pub opaque type Envelope(a) {
  Empty
  Filled(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil,
    List(a), #(String, Bool), fn(Int) -> a, label: Box(a))
}
pub fn main() { 42 }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let definition = plan
            .program
            .common
            .custom_types
            .definitions
            .iter()
            .find(|definition| definition.name.as_str() == "Envelope")
            .unwrap()
            .clone();
        assert_eq!(
            admit(std::slice::from_ref(&definition), &CustomGrants::new()),
            Ok(())
        );
        let name = CustomTypeName::new("geam".into(), "example".into(), "Envelope".into());
        let schema = HostCustomTypeSchema::new(
            "geam",
            "example",
            "Envelope",
            1,
            [
                HostCustomConstructorSchema::new("Empty", []),
                HostCustomConstructorSchema::new(
                    "Filled",
                    [
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::Int),
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::Float),
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::String),
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::BitArray),
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::UtfCodepoint),
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::Bool),
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::Nil),
                        HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::List(Box::new(HostSchemaType::Parameter(0))),
                        ),
                        HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::Tuple(Box::new([
                                HostSchemaType::String,
                                HostSchemaType::Bool,
                            ])),
                        ),
                        HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::Function {
                                arguments: Box::new([HostSchemaType::Int]),
                                return_: Box::new(HostSchemaType::Parameter(0)),
                            },
                        ),
                        HostCustomFieldSchema::new(
                            Some("label"),
                            HostSchemaType::custom(
                                "geam",
                                "example",
                                "Box",
                                [HostSchemaType::Parameter(0)],
                            ),
                        ),
                    ],
                ),
            ],
        )
        .with_shared_access(true);
        let grants = CustomGrants::from([(name.clone(), schema)]);
        let rejected = Err(NativeError::CustomGrant {
            custom_type: Box::new(name.clone()),
        });
        assert_eq!(admit(std::slice::from_ref(&definition), &grants), rejected);
        let mut shared = definition.clone();
        shared.native_access = Some(HostCustomAccess::Shared);
        assert_eq!(admit(std::slice::from_ref(&shared), &grants), Ok(()));
        assert_eq!(
            admit(std::slice::from_ref(&shared), &CustomGrants::new()),
            rejected
        );
        for change in [
            "arity",
            "lifetime",
            "constructor-count",
            "constructor-name",
            "field-count",
            "field-label",
            "field-type",
        ] {
            let mut changed = shared.clone();
            match change {
                "arity" => changed.parameters = 2,
                "lifetime" => changed.retention_lifetime = HostValueLifetime::Execution,
                _ => {
                    let mut constructors = changed.constructors.to_vec();
                    match change {
                        "constructor-count" => {
                            constructors.pop();
                        }
                        "constructor-name" => constructors[1].name = "Renamed".into(),
                        _ => {
                            let mut fields = constructors[1].fields.to_vec();
                            match change {
                                "field-count" => {
                                    fields.pop();
                                }
                                "field-label" => fields[0].label = Some("other".into()),
                                _ => fields[0].type_ = TypeMetadata::Bool,
                            }
                            constructors[1].fields = fields.into();
                        }
                    }
                    changed.constructors = constructors.into();
                }
            }
            assert_eq!(admit(&[changed], &grants), rejected, "{change}");
        }
        let retained_grants =
            CustomGrants::from([(name, HostCustomTypeSchema::retained::<Retained>())]);
        let mut retained = definition;
        retained.native_access = Some(HostCustomAccess::Retained);
        retained.retention_lifetime = HostValueLifetime::Execution;
        assert_eq!(admit(&[retained.clone()], &retained_grants), Ok(()));
        retained.native_access = Some(HostCustomAccess::Shared);
        assert_eq!(admit(&[retained], &retained_grants), rejected);
    }

    #[test]
    fn nested_external_fields_keep_their_complete_nominal_schema_and_arguments() {
        let schema = HostExternalTypeSchema::new("producer", "resources", "Token", 1);
        let original = HostSchemaType::External {
            schema: schema.clone(),
            arguments: Box::new([HostSchemaType::Parameter(0)]),
        };
        let nominal = NominalTypeMetadata {
            package: "producer".into(),
            module: "resources".into(),
            name: "Token".into(),
            arguments: vec![TypeMetadata::Parameter(TypeParameterId(0))].into(),
        };
        assert!(field(&TypeMetadata::External(nominal.clone()), &original));
        for changed in [
            NominalTypeMetadata {
                package: "other".into(),
                ..nominal.clone()
            },
            NominalTypeMetadata {
                module: "other".into(),
                ..nominal.clone()
            },
            NominalTypeMetadata {
                name: "Other".into(),
                ..nominal.clone()
            },
            NominalTypeMetadata {
                arguments: vec![TypeMetadata::Int].into(),
                ..nominal.clone()
            },
            NominalTypeMetadata {
                arguments: Vec::new().into(),
                ..nominal.clone()
            },
        ] {
            assert!(!field(&TypeMetadata::External(changed), &original));
        }
        assert!(!field(
            &TypeMetadata::External(nominal),
            &HostSchemaType::External {
                schema: HostExternalTypeSchema::new("producer", "resources", "Token", 2),
                arguments: Box::new([HostSchemaType::Parameter(0)])
            }
        ));
        assert!(!field(
            &TypeMetadata::Parameter(TypeParameterId(1)),
            &HostSchemaType::Parameter(0)
        ));
        assert!(!field(
            &TypeMetadata::List(Node::Owned(Box::new(TypeMetadata::Bool))),
            &HostSchemaType::List(Box::new(HostSchemaType::Int))
        ));
    }
}
