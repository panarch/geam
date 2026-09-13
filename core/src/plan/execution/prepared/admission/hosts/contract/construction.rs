mod native;
mod schema;

use super::{ContractError, Registration, Types};
use crate::host::{HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor};
use crate::plan::ValueType as Nominal;
use crate::plan::execution::host::construction::ConstructionIndex;
use crate::plan::execution::host::{HostConstructionTypes, HostedFunctionMetadata};
use crate::plan::execution::type_::{CustomTypeId, TypeMetadata, ValueType};
use std::collections::HashSet;

pub(super) fn admit(
    metadata: &HostedFunctionMetadata,
    registration: &Registration,
    arguments: &[Nominal],
    types: &Types<'_>,
) -> Result<(), ContractError> {
    let declarations = &registration.schema;
    let constructions = &metadata.constructions;
    index(&constructions.lists, ValueType::List, types)?;
    index(&constructions.customs, ValueType::Custom, types)?;
    index(&constructions.externals, ValueType::External, types)?;
    let mut walk = Walk {
        indexes: constructions,
        types,
        schemas: declarations
            .custom_schemas()
            .iter()
            .chain(registration.constructions.custom_schemas())
            .collect(),
        lists: HashSet::new(),
        customs: HashSet::new(),
        externals: HashSet::new(),
        expanded: HashSet::new(),
    };
    for descriptor in declarations
        .parameters()
        .iter()
        .chain([declarations.return_type()])
        .chain(registration.constructions.types())
    {
        walk.descriptor(descriptor, arguments)?;
    }
    if walk.lists.len() != constructions.lists.entries.len()
        || walk.customs.len() != constructions.customs.entries.len()
        || walk.externals.len() != constructions.externals.entries.len()
    {
        return Err(ContractError::Construction);
    }
    native::admit(metadata, registration, arguments, &walk.expanded, types)
}

fn index<Id: Copy>(
    index: &ConstructionIndex<Id>,
    value: impl Fn(Id) -> ValueType,
    types: &Types<'_>,
) -> Result<(), ContractError> {
    let mut previous: Option<&TypeMetadata> = None;
    for (key, id) in index.entries.iter() {
        if !types
            .matches(key, &value(*id))
            .map_err(ContractError::Type)?
            || previous.is_some_and(|previous| previous >= key)
        {
            return Err(ContractError::Construction);
        }
        previous = Some(key);
    }
    Ok(())
}

struct Walk<'a, 'data> {
    indexes: &'a HostConstructionTypes,
    types: &'a Types<'data>,
    schemas: Vec<&'a HostCustomTypeSchema>,
    lists: HashSet<usize>,
    customs: HashSet<usize>,
    externals: HashSet<usize>,
    expanded: HashSet<CustomTypeId>,
}

impl Walk<'_, '_> {
    fn descriptor(
        &mut self,
        descriptor: &HostTypeDescriptor,
        arguments: &[Nominal],
    ) -> Result<(), ContractError> {
        let value = descriptor.resolve_sealed(&|index| arguments[index].clone());
        self.record(&value)?;
        match descriptor {
            HostTypeDescriptor::List(item) => self.descriptor(item, arguments)?,
            HostTypeDescriptor::Tuple(items) => {
                for item in items {
                    self.descriptor(item, arguments)?;
                }
            }
            HostTypeDescriptor::Function {
                arguments: inputs,
                return_,
            } => {
                for input in inputs {
                    self.descriptor(input, arguments)?;
                }
                self.descriptor(return_, arguments)?;
            }
            HostTypeDescriptor::Custom {
                schema,
                arguments: inputs,
            } => {
                for input in inputs {
                    self.descriptor(input, arguments)?;
                }
                self.custom(schema, &value)?;
            }
            HostTypeDescriptor::External {
                arguments: inputs, ..
            } => {
                for input in inputs {
                    self.descriptor(input, arguments)?;
                }
            }
            HostTypeDescriptor::Parameter(_)
            | HostTypeDescriptor::OpaqueFunction { .. }
            | HostTypeDescriptor::Int
            | HostTypeDescriptor::Float
            | HostTypeDescriptor::String
            | HostTypeDescriptor::BitArray
            | HostTypeDescriptor::UtfCodepoint
            | HostTypeDescriptor::Bool
            | HostTypeDescriptor::Nil => {}
        }
        Ok(())
    }

    fn schema(
        &mut self,
        field: &HostSchemaType,
        arguments: &[Nominal],
    ) -> Result<(), ContractError> {
        let value = schema::resolve(field, arguments)?;
        self.record(&value)?;
        match field {
            HostSchemaType::List(item) => self.schema(item, arguments)?,
            HostSchemaType::Tuple(items) => {
                for item in items {
                    self.schema(item, arguments)?;
                }
            }
            HostSchemaType::Function {
                arguments: inputs,
                return_,
            } => {
                for input in inputs {
                    self.schema(input, arguments)?;
                }
                self.schema(return_, arguments)?;
            }
            HostSchemaType::Custom {
                package,
                module,
                name,
                arguments: inputs,
            } => {
                for input in inputs {
                    self.schema(input, arguments)?;
                }
                let schema = self
                    .schemas
                    .iter()
                    .rev()
                    .find(|schema| {
                        schema.package() == package
                            && schema.module() == module
                            && schema.name() == name
                    })
                    .copied()
                    .ok_or(ContractError::Construction)?;
                self.custom(schema, &value)?;
            }
            HostSchemaType::External {
                arguments: inputs, ..
            } => {
                for input in inputs {
                    self.schema(input, arguments)?;
                }
            }
            HostSchemaType::Parameter(_)
            | HostSchemaType::Int
            | HostSchemaType::Float
            | HostSchemaType::String
            | HostSchemaType::BitArray
            | HostSchemaType::UtfCodepoint
            | HostSchemaType::Bool
            | HostSchemaType::Nil => {}
        }
        Ok(())
    }

    fn custom(
        &mut self,
        schema: &HostCustomTypeSchema,
        nominal: &Nominal,
    ) -> Result<(), ContractError> {
        let (_, id) = find(&self.indexes.customs, nominal)?;
        if !self.expanded.insert(id) {
            return Ok(());
        }
        // The admitted index already fixes both the target and its nominal arguments.
        let stored = &self.types.customs.types[id.index()];
        let arguments = stored
            .type_
            .arguments
            .iter()
            .map(TypeMetadata::materialize)
            .collect::<Vec<_>>();
        if arguments.len() != schema.parameter_count()
            || stored.constructor_count != schema.constructors().len()
            || stored.constructors.len() != schema.constructors().len()
        {
            return Err(ContractError::Construction);
        }
        for (stored, original) in stored.constructors.iter().zip(schema.constructors()) {
            if stored.name.as_str() != original.name().as_str()
                || stored.fields.len() != original.fields().len()
            {
                return Err(ContractError::Construction);
            }
            for (stored, original) in stored.fields.iter().zip(original.fields()) {
                let value = schema::resolve(original.type_(), &arguments)?;
                if stored.label.as_ref().map(|label| label.as_str())
                    != original.label().map(|label| label.as_str())
                    || !self
                        .types
                        .metadata_matches_value(&TypeMetadata::from_public(&value), &stored.type_)
                    || !schema::refinement(&stored.refinement, original.type_())
                {
                    return Err(ContractError::Construction);
                }
                self.schema(original.type_(), &arguments)?;
            }
        }
        Ok(())
    }

    fn record(&mut self, type_: &Nominal) -> Result<(), ContractError> {
        match type_ {
            Nominal::List(_) => {
                self.lists.insert(find(&self.indexes.lists, type_)?.0);
            }
            Nominal::Custom(_) => {
                self.customs.insert(find(&self.indexes.customs, type_)?.0);
            }
            Nominal::External(_) => {
                self.externals
                    .insert(find(&self.indexes.externals, type_)?.0);
            }
            Nominal::Parameter(_)
            | Nominal::Int
            | Nominal::Float
            | Nominal::String
            | Nominal::BitArray
            | Nominal::UtfCodepoint
            | Nominal::Bool
            | Nominal::Nil
            | Nominal::Tuple(_)
            | Nominal::Function(_) => {}
        }
        Ok(())
    }
}

fn find<Id: Copy>(
    index: &ConstructionIndex<Id>,
    type_: &Nominal,
) -> Result<(usize, Id), ContractError> {
    let slot = index
        .entries
        .binary_search_by(|(key, _)| key.compare(type_))
        .map_err(|_| ContractError::Construction)?;
    Ok((slot, index.entries[slot].1))
}

#[cfg(test)]
mod tests {
    use super::{
        ConstructionIndex, ContractError, HostConstructionTypes, HostCustomTypeSchema,
        HostSchemaType, HostTypeDescriptor, Nominal, Registration, TypeMetadata, Types, ValueType,
        Walk, admit, index,
    };
    use crate::host::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostProviderModule, HostProviderSet,
        RegisteredHostConstructions, StatelessHostProfile,
    };
    use crate::plan::execution::prepared::admission::hosts::tests::lowered;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{
        CustomTypeId, ExternalTypeId, ListStorageTypeId, ListTypeId,
    };
    use std::collections::HashSet;

    #[test]
    fn construction_indexes_require_sorted_unique_nominal_keys_and_existing_targets() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Box(a) { Box(a) }
pub fn main() { #([42], ["text"], Box(42)) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let int = common
            .list_types
            .types
            .iter()
            .find(|value| matches!(value, ListStorageTypeId::Int(_)))
            .unwrap()
            .list_type();
        let string = common
            .list_types
            .types
            .iter()
            .find(|value| matches!(value, ListStorageTypeId::String(_)))
            .unwrap()
            .list_type();
        let int_key = TypeMetadata::List(Node::Static(&TypeMetadata::Int));
        let string_key = TypeMetadata::List(Node::Static(&TypeMetadata::String));
        let mut valid = vec![(int_key.clone(), int), (string_key, string)];
        valid.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            index(
                &ConstructionIndex {
                    entries: valid.clone().into()
                },
                ValueType::List,
                &types
            ),
            Ok(())
        );
        valid.reverse();
        for entries in [
            valid,
            vec![(int_key.clone(), int), (int_key.clone(), int)],
            vec![(int_key.clone(), string)],
        ] {
            assert_eq!(
                index(
                    &ConstructionIndex {
                        entries: entries.into()
                    },
                    ValueType::List,
                    &types
                ),
                Err(ContractError::Construction)
            );
        }
        assert_eq!(
            index(
                &ConstructionIndex {
                    entries: vec![(int_key, ListTypeId(99))].into()
                },
                ValueType::List,
                &types
            ),
            Err(ContractError::Type(
                super::super::super::super::type_::TypeError::MissingList { index: 99 }
            ))
        );
        assert_eq!(
            index(
                &ConstructionIndex {
                    entries: vec![(TypeMetadata::Int, CustomTypeId(99))].into()
                },
                ValueType::Custom,
                &types
            ),
            Err(ContractError::Type(
                super::super::super::super::type_::TypeError::MissingCustom { index: 99 }
            ))
        );
        assert_eq!(
            index(
                &ConstructionIndex {
                    entries: vec![(TypeMetadata::Int, ExternalTypeId(99))].into()
                },
                ValueType::External,
                &types
            ),
            Err(ContractError::Type(
                super::super::super::super::type_::TypeError::MissingExternal { index: 99 }
            ))
        );
    }

    #[test]
    fn declared_constructions_cover_each_retained_row_and_nested_generic_field() {
        let hosts = || {
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "app", "main",
            )
            .unwrap()
            .with_function("native", |value: num_bigint::BigInt| value)
            .unwrap()])
            .unwrap()
        };
        let (program, mut metadata, _) = lowered(
            r#"
pub type Box(a) { Box(a) }
@external(erlang, "native", "native")
fn native(value: Int) -> Int
pub fn main() { #([42], ["text"], [[42]], Box(42), Box([42]), Box(fn(x: Int) { x + 1 }), native(0)) }
"#,
            hosts(),
        );
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let (_, mut providers, _) = hosts().into_registered();
        let (schema, _, _) = providers.remove(0).functions.remove(0).into_parts();
        let box_schema = HostCustomTypeSchema::new(
            "app",
            "main",
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
        let callback = HostTypeDescriptor::Function {
            arguments: vec![HostTypeDescriptor::Int].into(),
            return_: Box::new(HostTypeDescriptor::Int),
        };
        let descriptors = vec![
            HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int)),
            HostTypeDescriptor::List(Box::new(HostTypeDescriptor::String)),
            HostTypeDescriptor::List(Box::new(HostTypeDescriptor::List(Box::new(
                HostTypeDescriptor::Int,
            )))),
            HostTypeDescriptor::Custom {
                schema: box_schema.clone(),
                arguments: vec![HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))].into(),
            },
            HostTypeDescriptor::Custom {
                schema: box_schema.clone(),
                arguments: vec![HostTypeDescriptor::Int].into(),
            },
            HostTypeDescriptor::Custom {
                schema: box_schema.clone(),
                arguments: vec![callback].into(),
            },
        ];
        let mut lists = common
            .list_types
            .types
            .iter()
            .map(|storage| {
                let id = storage.list_type();
                (
                    TypeMetadata::from_public(&common.list_types.list_value_type(
                        id,
                        &common.custom_types,
                        &common.external_types,
                    )),
                    id,
                )
            })
            .collect::<Vec<_>>();
        lists.sort_by(|a, b| a.0.cmp(&b.0));
        let mut customs = common
            .custom_types
            .types
            .iter()
            .enumerate()
            .map(|(id, value)| (TypeMetadata::Custom(value.type_.clone()), CustomTypeId(id)))
            .collect::<Vec<_>>();
        customs.sort_by(|a, b| a.0.cmp(&b.0));
        let indexes = HostConstructionTypes {
            lists: ConstructionIndex {
                entries: lists.into(),
            },
            customs: ConstructionIndex {
                entries: customs.into(),
            },
            externals: ConstructionIndex {
                entries: Table::Static(&[]),
            },
            natives: metadata[0].constructions.natives.clone(),
        };
        metadata[0].constructions = indexes.clone();
        let mut registration = Registration {
            schema,
            constructions: RegisteredHostConstructions::new(
                vec![HostTypeDescriptor::Tuple(descriptors.clone().into())].into_boxed_slice(),
                vec![box_schema.clone()].into_boxed_slice(),
            ),
        };
        assert_eq!(admit(&metadata[0], &registration, &[], &types), Ok(()));
        let invalid = &mut metadata[0];
        invalid.constructions.lists.entries =
            vec![(indexes.lists.entries[0].0.clone(), ListTypeId(99))].into();
        assert_eq!(
            admit(invalid, &registration, &[], &types),
            Err(ContractError::Type(
                super::super::super::super::type_::TypeError::MissingList { index: 99 }
            ))
        );
        invalid.constructions = indexes.clone();
        invalid.constructions.customs.entries =
            vec![(indexes.customs.entries[0].0.clone(), CustomTypeId(99))].into();
        assert_eq!(
            admit(invalid, &registration, &[], &types),
            Err(ContractError::Type(
                super::super::super::super::type_::TypeError::MissingCustom { index: 99 }
            ))
        );
        invalid.constructions = indexes.clone();
        invalid.constructions.externals.entries =
            vec![(TypeMetadata::Int, ExternalTypeId(99))].into();
        assert_eq!(
            admit(invalid, &registration, &[], &types),
            Err(ContractError::Type(
                super::super::super::super::type_::TypeError::MissingExternal { index: 99 }
            ))
        );
        invalid.constructions = indexes.clone();
        registration.constructions = RegisteredHostConstructions::empty();
        assert_eq!(
            admit(&metadata[0], &registration, &[], &types),
            Err(ContractError::Construction)
        );
        registration.constructions = RegisteredHostConstructions::new(
            vec![HostTypeDescriptor::Tuple(descriptors.into())].into_boxed_slice(),
            vec![box_schema.clone()].into_boxed_slice(),
        );
        metadata[0].constructions.lists.entries = Table::Static(&[]);
        assert_eq!(
            admit(&metadata[0], &registration, &[], &types),
            Err(ContractError::Construction)
        );
        metadata[0].constructions = indexes.clone();
        metadata[0].constructions.customs.entries = Table::Static(&[]);
        assert_eq!(
            admit(&metadata[0], &registration, &[], &types),
            Err(ContractError::Construction)
        );

        let walk = || Walk {
            indexes: &indexes,
            types: &types,
            schemas: vec![&box_schema],
            lists: HashSet::new(),
            customs: HashSet::new(),
            externals: HashSet::new(),
            expanded: HashSet::new(),
        };
        let box_field =
            HostSchemaType::custom("app", "main", "Box", [HostSchemaType::Parameter(0)]);
        assert_eq!(walk().schema(&box_field, &[Nominal::Int]), Ok(()));
        assert_eq!(
            walk().schema(
                &HostSchemaType::list(HostSchemaType::Parameter(0)),
                &[Nominal::Int]
            ),
            Ok(())
        );
        assert_eq!(
            walk().schema(
                &HostSchemaType::Tuple(vec![HostSchemaType::Int, HostSchemaType::String].into()),
                &[]
            ),
            Ok(())
        );
        assert_eq!(
            walk().schema(
                &HostSchemaType::Function {
                    arguments: vec![HostSchemaType::Int].into(),
                    return_: Box::new(HostSchemaType::String)
                },
                &[]
            ),
            Ok(())
        );
        assert_eq!(
            walk().schema(&box_field, &[]),
            Err(ContractError::TypeArguments)
        );
        let mut missing = walk();
        missing.schemas.clear();
        assert_eq!(
            missing.schema(&box_field, &[Nominal::Int]),
            Err(ContractError::Construction)
        );
        let nominal = Nominal::Custom(crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("app".into(), "main".into(), "Box".into()),
            vec![Nominal::Int],
        ));
        for (parameters, constructors, expected) in [
            (
                0,
                box_schema.constructors().to_vec(),
                ContractError::Construction,
            ),
            (1, Vec::new(), ContractError::Construction),
            (
                1,
                vec![HostCustomConstructorSchema::new(
                    "Different",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::Parameter(0),
                    )],
                )],
                ContractError::Construction,
            ),
            (
                1,
                vec![HostCustomConstructorSchema::new("Box", [])],
                ContractError::Construction,
            ),
            (
                1,
                vec![HostCustomConstructorSchema::new(
                    "Box",
                    [HostCustomFieldSchema::new(
                        Some("value"),
                        HostSchemaType::Parameter(0),
                    )],
                )],
                ContractError::Construction,
            ),
            (
                1,
                vec![HostCustomConstructorSchema::new(
                    "Box",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::Bool,
                    )],
                )],
                ContractError::Construction,
            ),
            (
                1,
                vec![HostCustomConstructorSchema::new(
                    "Box",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::Int,
                    )],
                )],
                ContractError::Construction,
            ),
            (
                1,
                vec![HostCustomConstructorSchema::new(
                    "Box",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::Parameter(1),
                    )],
                )],
                ContractError::TypeArguments,
            ),
        ] {
            let schema = HostCustomTypeSchema::new("app", "main", "Box", parameters, constructors);
            assert_eq!(walk().custom(&schema, &nominal), Err(expected));
        }
        let mut incomplete = indexes.clone();
        incomplete.lists.entries = indexes
            .lists
            .entries
            .iter()
            .filter(|(type_, _)| type_ != &TypeMetadata::List(Node::Static(&TypeMetadata::Int)))
            .cloned()
            .collect();
        for descriptor in [
            HostTypeDescriptor::List(Box::new(HostTypeDescriptor::List(Box::new(
                HostTypeDescriptor::Int,
            )))),
            HostTypeDescriptor::Tuple(
                vec![HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))].into(),
            ),
            HostTypeDescriptor::Function {
                arguments: vec![HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))].into(),
                return_: Box::new(HostTypeDescriptor::Int),
            },
            HostTypeDescriptor::Function {
                arguments: vec![HostTypeDescriptor::Int].into(),
                return_: Box::new(HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))),
            },
            HostTypeDescriptor::Custom {
                schema: box_schema.clone(),
                arguments: vec![HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))].into(),
            },
        ] {
            let mut missing = walk();
            missing.indexes = &incomplete;
            assert_eq!(
                missing.descriptor(&descriptor, &[]),
                Err(ContractError::Construction)
            );
        }
        for schema in [
            HostSchemaType::list(HostSchemaType::list(HostSchemaType::Int)),
            HostSchemaType::Tuple(vec![HostSchemaType::list(HostSchemaType::Int)].into()),
            HostSchemaType::Function {
                arguments: vec![HostSchemaType::list(HostSchemaType::Int)].into(),
                return_: Box::new(HostSchemaType::Int),
            },
            HostSchemaType::Function {
                arguments: vec![HostSchemaType::Int].into(),
                return_: Box::new(HostSchemaType::list(HostSchemaType::Int)),
            },
            HostSchemaType::custom(
                "app",
                "main",
                "Box",
                [HostSchemaType::list(HostSchemaType::Int)],
            ),
        ] {
            let mut missing = walk();
            missing.indexes = &incomplete;
            assert_eq!(
                missing.schema(&schema, &[]),
                Err(ContractError::Construction)
            );
        }
        let mut missing_field = walk();
        missing_field.indexes = &incomplete;
        let box_list = Nominal::Custom(crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("app".into(), "main".into(), "Box".into()),
            vec![Nominal::List(Box::new(Nominal::Int))],
        ));
        assert_eq!(
            missing_field.custom(&box_schema, &box_list),
            Err(ContractError::Construction)
        );
        let empty_box = HostCustomTypeSchema::new("app", "main", "Box", 1, []);
        let mut missing_constructor = walk();
        missing_constructor.schemas = vec![&empty_box];
        assert_eq!(
            missing_constructor.schema(&box_field, &[Nominal::Int]),
            Err(ContractError::Construction)
        );
        let missing_custom = Nominal::Custom(crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("app".into(), "main".into(), "Box".into()),
            vec![Nominal::Bool],
        ));
        assert_eq!(
            walk().custom(&box_schema, &missing_custom),
            Err(ContractError::Construction)
        );
        let mut incomplete = walk();
        incomplete.indexes = &indexes;
        assert_eq!(
            incomplete.descriptor(
                &HostTypeDescriptor::Custom {
                    schema: HostCustomTypeSchema::new("app", "main", "Box", 1, []),
                    arguments: vec![HostTypeDescriptor::Int].into(),
                },
                &[]
            ),
            Err(ContractError::Construction)
        );
        assert_eq!(
            walk().schema(
                &HostSchemaType::custom("app", "main", "Box", [HostSchemaType::Int]),
                &[]
            ),
            Ok(())
        );
    }

    #[test]
    fn external_construction_arguments_require_their_own_registered_containers() {
        use crate::plan::execution::type_::{ExternalTypeTable, NominalTypeMetadata};
        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { [42] }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let nominal = NominalTypeMetadata {
            package: "app".into(),
            module: "main".into(),
            name: "Token".into(),
            arguments: vec![TypeMetadata::List(Node::Static(&TypeMetadata::Int))].into(),
        };
        let externals = ExternalTypeTable {
            types: vec![nominal.clone()].into(),
        };
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &externals,
            &common.value_shapes,
        )
        .unwrap();
        let list = common.list_types.types[0].list_type();
        let indexes = HostConstructionTypes {
            lists: ConstructionIndex {
                entries: vec![(TypeMetadata::List(Node::Static(&TypeMetadata::Int)), list)].into(),
            },
            customs: ConstructionIndex {
                entries: Table::Static(&[]),
            },
            externals: ConstructionIndex {
                entries: vec![(TypeMetadata::External(nominal), ExternalTypeId(0))].into(),
            },
            natives: crate::plan::execution::host::NativeConversions {
                roots: Table::Static(&[]),
                nodes: Table::Static(&[]),
            },
        };
        let descriptor = HostTypeDescriptor::External {
            schema: crate::host::HostExternalTypeSchema::new("app", "main", "Token", 1),
            arguments: vec![HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))].into(),
        };
        let schema = HostSchemaType::External {
            schema: crate::host::HostExternalTypeSchema::new("app", "main", "Token", 1),
            arguments: vec![HostSchemaType::list(HostSchemaType::Int)].into(),
        };
        for missing in [None, Some("list"), Some("external")] {
            let mut selected = indexes.clone();
            match missing {
                Some("list") => selected.lists.entries = Table::Static(&[]),
                Some(_) => selected.externals.entries = Table::Static(&[]),
                None => {}
            }
            let mut walk = Walk {
                indexes: &selected,
                types: &types,
                schemas: Vec::new(),
                lists: HashSet::new(),
                customs: HashSet::new(),
                externals: HashSet::new(),
                expanded: HashSet::new(),
            };
            let expected = missing.map_or(Ok(()), |_| Err(ContractError::Construction));
            assert_eq!(walk.descriptor(&descriptor, &[]), expected);
            assert_eq!(walk.schema(&schema, &[]), expected);
        }
    }
}
