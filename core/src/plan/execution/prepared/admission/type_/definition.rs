use super::{TypeError, Types};
use crate::plan::execution::storage::Node;
use crate::plan::execution::type_::custom::{
    CustomDefinition, FieldRefinement, definition::RESULT,
};
use crate::plan::execution::type_::{
    CustomTypeDescriptor, FunctionMetadata, NominalTypeMetadata, TypeMetadata,
};

impl<'data> Types<'data> {
    pub(in crate::plan::execution::prepared::admission) fn definition_of(
        &self,
        id: crate::plan::execution::type_::CustomTypeId,
    ) -> &'data CustomDefinition {
        let type_ = &self.customs.types[id.index()].type_;
        self.declarations[&(
            type_.package.as_str(),
            type_.module.as_str(),
            type_.name.as_str(),
        )]
    }

    pub(in crate::plan::execution::prepared::admission) fn definition(
        &self,
        type_: &NominalTypeMetadata,
    ) -> Result<&'data CustomDefinition, TypeError> {
        let definition = self
            .declarations
            .get(&(
                type_.package.as_str(),
                type_.module.as_str(),
                type_.name.as_str(),
            ))
            .copied()
            .ok_or_else(|| TypeError::MissingDefinition {
                package: type_.package.to_string(),
                module: type_.module.to_string(),
                name: type_.name.to_string(),
            })?;
        if definition.parameters != type_.arguments.len() {
            return Err(TypeError::Definition);
        }
        Ok(definition)
    }

    pub(super) fn definitions(&self) -> Result<(), TypeError> {
        let mut previous = None;
        for definition in self.customs.definitions.iter() {
            let identity = definition.identity();
            if identity == RESULT.identity()
                || previous.is_some_and(|previous| previous >= identity)
            {
                return Err(TypeError::Definition);
            }
            previous = Some(identity);
            let mut names = std::collections::HashSet::new();
            for constructor in definition.constructors.iter() {
                if !names.insert(constructor.name.as_str()) {
                    return Err(TypeError::Definition);
                }
                for field in constructor.fields.iter() {
                    self.metadata(&field.type_)?;
                    let mut pending = vec![&field.type_];
                    while let Some(type_) = pending.pop() {
                        match type_ {
                            TypeMetadata::Parameter(id) if id.0 >= definition.parameters => {
                                return Err(TypeError::DefinitionParameter {
                                    index: id.0,
                                    parameters: definition.parameters,
                                });
                            }
                            TypeMetadata::Tuple(items) => pending.extend(items.iter()),
                            TypeMetadata::List(item) => pending.push(item.as_ref()),
                            TypeMetadata::Function(function) => {
                                pending.extend(function.arguments.iter());
                                pending.push(function.return_.as_ref());
                            }
                            TypeMetadata::Custom(nominal) => {
                                self.definition(nominal)?;
                                pending.extend(nominal.arguments.iter());
                            }
                            TypeMetadata::External(nominal) => {
                                pending.extend(nominal.arguments.iter())
                            }
                            TypeMetadata::Parameter(_)
                            | TypeMetadata::Int
                            | TypeMetadata::Float
                            | TypeMetadata::String
                            | TypeMetadata::BitArray
                            | TypeMetadata::UtfCodepoint
                            | TypeMetadata::Bool
                            | TypeMetadata::Nil => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn custom_definition(
        &self,
        descriptor: &CustomTypeDescriptor,
    ) -> Result<(), TypeError> {
        let definition = self.definition(&descriptor.type_)?;
        if definition.constructors.len() != descriptor.constructor_count {
            return Err(TypeError::Definition);
        }
        for constructor in descriptor.constructors.iter() {
            let original = &definition.constructors[constructor.id.index];
            if constructor.name != original.name
                || constructor.native_tag.as_str()
                    != gleam_compiler_core::strings::to_snake_case(&original.name).as_str()
                || constructor.fields.len() != original.fields.len()
            {
                return Err(TypeError::Definition);
            }
            for (field, original) in constructor.fields.iter().zip(original.fields.iter()) {
                if field.label != original.label
                    || !refinement(&field.refinement, &original.type_)
                    || !self.metadata_matches_value(
                        &substitute(&original.type_, &descriptor.type_.arguments),
                        &field.type_,
                    )
                {
                    return Err(TypeError::Definition);
                }
            }
        }
        Ok(())
    }
}

fn substitute(template: &TypeMetadata, arguments: &[TypeMetadata]) -> TypeMetadata {
    match template {
        TypeMetadata::Parameter(id) => arguments[id.0].clone(),
        TypeMetadata::Tuple(items) => TypeMetadata::Tuple(
            items
                .iter()
                .map(|item| substitute(item, arguments))
                .collect(),
        ),
        TypeMetadata::List(item) => {
            TypeMetadata::List(Node::Owned(Box::new(substitute(item, arguments))))
        }
        TypeMetadata::Function(function) => TypeMetadata::Function(FunctionMetadata {
            arguments: function
                .arguments
                .iter()
                .map(|item| substitute(item, arguments))
                .collect(),
            return_: Node::Owned(Box::new(substitute(&function.return_, arguments))),
        }),
        TypeMetadata::Custom(nominal) => {
            TypeMetadata::Custom(nominal_substitution(nominal, arguments))
        }
        TypeMetadata::External(nominal) => {
            TypeMetadata::External(nominal_substitution(nominal, arguments))
        }
        TypeMetadata::Int
        | TypeMetadata::Float
        | TypeMetadata::String
        | TypeMetadata::BitArray
        | TypeMetadata::UtfCodepoint
        | TypeMetadata::Bool
        | TypeMetadata::Nil => template.clone(),
    }
}

fn nominal_substitution(
    nominal: &NominalTypeMetadata,
    arguments: &[TypeMetadata],
) -> NominalTypeMetadata {
    NominalTypeMetadata {
        package: nominal.package.clone(),
        module: nominal.module.clone(),
        name: nominal.name.clone(),
        arguments: nominal
            .arguments
            .iter()
            .map(|item| substitute(item, arguments))
            .collect(),
    }
}

fn refinement(rule: &FieldRefinement, template: &TypeMetadata) -> bool {
    match (rule, template) {
        (FieldRefinement::Argument(index), TypeMetadata::Parameter(id)) => *index == id.0,
        (FieldRefinement::Tuple(rules), TypeMetadata::Tuple(items)) => {
            rules.len() == items.len()
                && rules
                    .iter()
                    .zip(items.iter())
                    .all(|(rule, item)| refinement(rule, item))
        }
        (FieldRefinement::List(rule), TypeMetadata::List(item)) => refinement(rule, item),
        (FieldRefinement::Function { arguments, return_ }, TypeMetadata::Function(function)) => {
            arguments.len() == function.arguments.len()
                && arguments
                    .iter()
                    .zip(function.arguments.iter())
                    .all(|(rule, item)| refinement(rule, item))
                && refinement(return_, &function.return_)
        }
        (FieldRefinement::Custom(rules), TypeMetadata::Custom(nominal)) => {
            rules.len() == nominal.arguments.len()
                && rules
                    .iter()
                    .zip(nominal.arguments.iter())
                    .all(|(rule, item)| refinement(rule, item))
        }
        (
            FieldRefinement::Value,
            TypeMetadata::Int
            | TypeMetadata::Float
            | TypeMetadata::String
            | TypeMetadata::BitArray
            | TypeMetadata::UtfCodepoint
            | TypeMetadata::Bool
            | TypeMetadata::Nil
            | TypeMetadata::External(_),
        ) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomDefinition, Node, TypeError, TypeMetadata, Types};
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::custom::{ConstructorDefinition, FieldDefinition};

    #[test]
    fn rejects_lowered_constructor_metadata_that_disagrees_with_its_declaration() {
        use crate::plan::execution::type_::CustomConstructorDescriptor;
        use crate::plan::execution::type_::custom::FieldRefinement;

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub type Box(a) { Box(item: a) }\npub fn main() { Box(42) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        static CYCLE: FieldRefinement = FieldRefinement::List(Node::Static(&CYCLE));
        let mutations: [fn(&mut CustomConstructorDescriptor); 6] = [
            |constructor| constructor.native_tag = "wrong".into(),
            |constructor| constructor.name = "Other".into(),
            |constructor| constructor.fields = Table::Static(&[]),
            |constructor| {
                let mut fields = constructor.fields.clone().into_vec();
                fields[0].refinement = FieldRefinement::Argument(1);
                constructor.fields = fields.into();
            },
            |constructor| {
                let mut fields = constructor.fields.clone().into_vec();
                fields[0].refinement = CYCLE.clone();
                constructor.fields = fields.into();
            },
            |constructor| {
                let mut fields = constructor.fields.clone().into_vec();
                fields[0].type_ = crate::plan::execution::type_::ValueType::Bool;
                constructor.fields = fields.into();
            },
        ];
        for mutate in mutations {
            let mut customs = common.custom_types.as_ref().clone();
            let mut descriptors = customs.types.into_vec();
            let mut constructors = descriptors[0].constructors.clone().into_vec();
            mutate(&mut constructors[0]);
            descriptors[0].constructors = constructors.into();
            customs.types = descriptors.into();
            assert_eq!(
                Types::admit(
                    &common.list_types,
                    &customs,
                    &common.external_types,
                    &common.value_shapes,
                )
                .err(),
                Some(TypeError::Definition)
            );
        }
    }

    #[test]
    fn preserves_complete_definitions_and_rejects_malformed_declaration_graphs() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Box(a) { Box(item: a) }
pub fn main() { Box(42) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let original = &common.custom_types;
        let definition = &original.definitions[0];
        let types = Types::admit(
            &common.list_types,
            original,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        assert!(std::ptr::eq(
            types.definition_of(crate::plan::execution::type_::CustomTypeId(0)),
            definition
        ));
        assert_eq!(definition.identity(), ("geam", "example", "Box"));
        assert_eq!(definition.parameters, 1);
        assert_eq!(
            definition.constructors.as_ref(),
            &[ConstructorDefinition {
                name: "Box".into(),
                fields: vec![FieldDefinition {
                    label: Some("item".into()),
                    type_: TypeMetadata::Parameter(crate::plan::TypeParameterId(0))
                }]
                .into(),
            }]
        );
        type DefinitionMutation = fn(&mut CustomDefinition);
        let cases: [(DefinitionMutation, TypeError); 6] = [
            (
                |definition| definition.parameters = 0,
                TypeError::DefinitionParameter {
                    index: 0,
                    parameters: 0,
                },
            ),
            (
                |definition| definition.constructors = Table::Static(&[]),
                TypeError::Definition,
            ),
            (
                |definition| definition.parameters = 2,
                TypeError::Definition,
            ),
            (
                |definition| {
                    definition.constructors = vec![
                        definition.constructors[0].clone(),
                        definition.constructors[0].clone(),
                    ]
                    .into();
                },
                TypeError::Definition,
            ),
            (
                |definition| {
                    let mut constructors = definition.constructors.clone().into_vec();
                    constructors[0].fields = vec![FieldDefinition {
                        label: None,
                        type_: TypeMetadata::Custom(
                            crate::plan::execution::type_::NominalTypeMetadata {
                                package: "geam".into(),
                                module: "example".into(),
                                name: "Missing".into(),
                                arguments: Table::Static(&[]),
                            },
                        ),
                    }]
                    .into();
                    definition.constructors = constructors.into();
                },
                TypeError::MissingDefinition {
                    package: "geam".into(),
                    module: "example".into(),
                    name: "Missing".into(),
                },
            ),
            (
                |definition| {
                    let mut constructors = definition.constructors.clone().into_vec();
                    constructors[0].fields = vec![FieldDefinition {
                        label: None,
                        type_: TypeMetadata::Bool,
                    }]
                    .into();
                    definition.constructors = constructors.into();
                },
                TypeError::Definition,
            ),
        ];
        for (mutate, expected) in cases {
            let mut customs = original.as_ref().clone();
            let mut definition = definition.clone();
            mutate(&mut definition);
            customs.definitions = vec![definition].into();
            assert_eq!(
                Types::admit(
                    &common.list_types,
                    &customs,
                    &common.external_types,
                    &common.value_shapes
                )
                .err(),
                Some(expected)
            );
        }
        let mut customs = original.as_ref().clone();
        customs.definitions = vec![definition.clone(), definition.clone()].into();
        assert_eq!(
            Types::admit(
                &common.list_types,
                &customs,
                &common.external_types,
                &common.value_shapes
            )
            .err(),
            Some(TypeError::Definition)
        );
        customs.definitions = Table::Static(&[]);
        assert_eq!(
            Types::admit(
                &common.list_types,
                &customs,
                &common.external_types,
                &common.value_shapes
            )
            .err(),
            Some(TypeError::MissingDefinition {
                package: "geam".into(),
                module: "example".into(),
                name: "Box".into(),
            })
        );
        static CYCLE: TypeMetadata = TypeMetadata::List(Node::Static(&CYCLE));
        let mut definition = definition.clone();
        definition.constructors = vec![ConstructorDefinition {
            name: "Box".into(),
            fields: vec![FieldDefinition {
                label: None,
                type_: CYCLE.clone(),
            }]
            .into(),
        }]
        .into();
        customs.definitions = vec![definition].into();
        assert_eq!(
            Types::admit(
                &common.list_types,
                &customs,
                &common.external_types,
                &common.value_shapes
            )
            .err(),
            Some(TypeError::RecursiveMetadata)
        );
    }
}
