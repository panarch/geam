use super::custom::{CustomDefinition, definition::RESULT};
use super::{CustomTypeTable, ExternalTypeTable, ListTypeTable, TypeMetadata};
use crate::host::HostValueLifetime;
use crate::plan::execution::host::registration::ExternalSchema;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use std::collections::{BTreeMap, BTreeSet};

type Identity = (String, String, String);

/// Finite type dependencies used only while sealing or admitting type tables.
/// No value, payload, or execution endpoint participates in this analysis.
pub(in crate::plan::execution) struct ValueLifetimes {
    customs: BTreeMap<Identity, Dependencies>,
    externals: BTreeMap<Identity, HostValueLifetime>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Dependencies {
    execution: bool,
    parameters: BTreeSet<usize>,
}

impl ValueLifetimes {
    pub(in crate::plan::execution) fn new(
        definitions: &[CustomDefinition],
        externals: &[ExternalSchema],
    ) -> Self {
        let mut this = Self {
            customs: definitions
                .iter()
                .chain(std::iter::once(&RESULT))
                .map(|definition| {
                    (
                        identity(definition.identity()),
                        Dependencies {
                            execution: definition.retention_lifetime.requires_execution(),
                            parameters: BTreeSet::new(),
                        },
                    )
                })
                .collect(),
            externals: externals
                .iter()
                .map(|schema| {
                    (
                        identity((
                            schema.package.as_str(),
                            schema.module.as_str(),
                            schema.name.as_str(),
                        )),
                        schema.lifetime,
                    )
                })
                .collect(),
        };
        // Dependencies only grow. Recursive source types do not expand their
        // type arguments and therefore terminate even for Nest(List(a)).
        loop {
            let mut changed = false;
            for definition in definitions.iter().chain(std::iter::once(&RESULT)) {
                let mut dependencies = this.customs[&identity(definition.identity())].clone();
                for constructor in definition.constructors.iter() {
                    for field in constructor.fields.iter() {
                        dependencies.include(this.dependencies(&field.type_));
                    }
                }
                let key = identity(definition.identity());
                if this.customs[&key] != dependencies {
                    this.customs.insert(key, dependencies);
                    changed = true;
                }
            }
            if !changed {
                return this;
            }
        }
    }

    pub(in crate::plan::execution) fn seal(
        &self,
        lists: &mut ListTypeTable,
        customs: &mut CustomTypeTable,
        externals: &mut ExternalTypeTable,
    ) {
        let custom_lifetimes = customs
            .types
            .iter()
            .map(|type_| self.lifetime(&TypeMetadata::Custom(type_.type_.clone())))
            .collect::<Vec<_>>();
        customs.types = std::mem::replace(&mut customs.types, Vec::new().into())
            .into_vec()
            .into_iter()
            .zip(custom_lifetimes)
            .map(|(mut type_, lifetime)| {
                type_.lifetime = lifetime;
                type_
            })
            .collect();
        externals.lifetimes = externals
            .types
            .iter()
            .map(|type_| self.lifetime(&TypeMetadata::External(type_.clone())))
            .collect();
        lists.lifetimes = (0..lists.types.len())
            .map(|index| {
                let type_ = lists.list_value_type(super::ListTypeId(index), customs, externals);
                self.lifetime(&TypeMetadata::from_public(&type_))
            })
            .collect();
    }

    pub(in crate::plan::execution) fn matches(
        &self,
        lists: &ListTypeTable,
        customs: &CustomTypeTable,
        externals: &ExternalTypeTable,
    ) -> bool {
        lists.lifetimes.len() == lists.types.len()
            && externals.lifetimes.len() == externals.types.len()
            && customs.types.iter().all(|type_| {
                type_.lifetime == self.lifetime(&TypeMetadata::Custom(type_.type_.clone()))
            })
            && externals
                .types
                .iter()
                .zip(externals.lifetimes.iter())
                .all(|(type_, lifetime)| {
                    *lifetime == self.lifetime(&TypeMetadata::External(type_.clone()))
                })
            && lists.lifetimes.iter().enumerate().all(|(index, lifetime)| {
                let type_ = lists.list_value_type(super::ListTypeId(index), customs, externals);
                *lifetime == self.lifetime(&TypeMetadata::from_public(&type_))
            })
    }

    fn lifetime(&self, type_: &TypeMetadata) -> HostValueLifetime {
        if self.dependencies(type_).execution {
            HostValueLifetime::Execution
        } else {
            HostValueLifetime::LoadedOwner
        }
    }

    fn dependencies(&self, type_: &TypeMetadata) -> Dependencies {
        let mut dependencies = Dependencies::default();
        match type_ {
            TypeMetadata::Parameter(parameter) => {
                dependencies.parameters.insert(parameter.0);
            }
            TypeMetadata::Function(_) => dependencies.execution = true,
            TypeMetadata::Tuple(items) => {
                for item in items.iter() {
                    dependencies.include(self.dependencies(item));
                }
            }
            TypeMetadata::List(item) => dependencies.include(self.dependencies(item)),
            TypeMetadata::Custom(type_) => {
                let template = &self.customs[&identity((
                    type_.package.as_str(),
                    type_.module.as_str(),
                    type_.name.as_str(),
                ))];
                dependencies.execution = template.execution;
                for parameter in &template.parameters {
                    dependencies.include(self.dependencies(&type_.arguments[*parameter]));
                }
            }
            TypeMetadata::External(type_) => {
                dependencies.execution = self.externals[&identity((
                    type_.package.as_str(),
                    type_.module.as_str(),
                    type_.name.as_str(),
                ))]
                    .requires_execution();
                for argument in type_.arguments.iter() {
                    dependencies.include(self.dependencies(argument));
                }
            }
            TypeMetadata::Int
            | TypeMetadata::Float
            | TypeMetadata::String
            | TypeMetadata::BitArray
            | TypeMetadata::UtfCodepoint
            | TypeMetadata::Bool
            | TypeMetadata::Nil => {}
        }
        dependencies
    }
}

impl Dependencies {
    fn include(&mut self, other: Self) {
        self.execution |= other.execution;
        self.parameters.extend(other.parameters);
    }
}

impl Emit for HostValueLifetime {
    fn emit(&self, output: &mut Rust) {
        output.path(match self {
            Self::LoadedOwner => "host::HostValueLifetime::LoadedOwner",
            Self::Execution => "host::HostValueLifetime::Execution",
        });
    }
}

fn identity((package, module, name): (&str, &str, &str)) -> Identity {
    (package.to_owned(), module.to_owned(), name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::ValueLifetimes;
    use crate::host::HostValueLifetime;
    use crate::plan::execution::host::registration::ExternalSchema;
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::storage::Node;
    use crate::plan::execution::type_::{FunctionMetadata, NominalTypeMetadata, TypeMetadata};

    #[test]
    fn closes_recursive_and_phantom_parameters_without_scanning_values() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Nest(a) { Bottom(a) Nested(Nest(List(a))) }
pub type Phantom(a) { Phantom }
pub opaque type Secret { Data(Int) Callback(fn(Int) -> Int) }
pub fn main() { 42 }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let definitions = &plan.program.common.custom_types.definitions;
        let lifetimes = ValueLifetimes::new(definitions, &[]);
        let function = TypeMetadata::Function(FunctionMetadata {
            arguments: vec![TypeMetadata::Int].into(),
            return_: Node::Owned(Box::new(TypeMetadata::Int)),
        });
        for (name, argument, expected) in [
            ("Nest", TypeMetadata::Int, HostValueLifetime::LoadedOwner),
            ("Nest", function.clone(), HostValueLifetime::Execution),
            ("Phantom", function.clone(), HostValueLifetime::LoadedOwner),
        ] {
            let type_ = TypeMetadata::Custom(NominalTypeMetadata {
                package: "geam".into(),
                module: "example".into(),
                name: name.into(),
                arguments: vec![argument].into(),
            });
            assert_eq!(lifetimes.lifetime(&type_), expected);
        }
        let secret = TypeMetadata::Custom(NominalTypeMetadata {
            package: "geam".into(),
            module: "example".into(),
            name: "Secret".into(),
            arguments: Vec::new().into(),
        });
        assert_eq!(lifetimes.lifetime(&secret), HostValueLifetime::Execution);
        assert_eq!(
            lifetimes.lifetime(&TypeMetadata::List(Node::Owned(Box::new(function)))),
            HostValueLifetime::Execution
        );
        for scalar in [
            TypeMetadata::Int,
            TypeMetadata::Float,
            TypeMetadata::String,
            TypeMetadata::BitArray,
            TypeMetadata::UtfCodepoint,
            TypeMetadata::Bool,
            TypeMetadata::Nil,
        ] {
            assert_eq!(lifetimes.lifetime(&scalar), HostValueLifetime::LoadedOwner);
        }
    }

    #[test]
    fn external_producer_floor_and_type_arguments_both_contribute() {
        let definitions = [
            ExternalSchema {
                package: "storage".into(),
                module: "storage".into(),
                name: "Data".into(),
                parameter_count: 1,
                lifetime: HostValueLifetime::LoadedOwner,
            },
            ExternalSchema {
                package: "storage".into(),
                module: "storage".into(),
                name: "Work".into(),
                parameter_count: 1,
                lifetime: HostValueLifetime::Execution,
            },
        ];
        let lifetimes = ValueLifetimes::new(&[], &definitions);
        let function = TypeMetadata::Function(FunctionMetadata {
            arguments: vec![TypeMetadata::Int].into(),
            return_: Node::Owned(Box::new(TypeMetadata::Int)),
        });
        for (name, argument, expected) in [
            ("Data", TypeMetadata::Int, HostValueLifetime::LoadedOwner),
            ("Data", function, HostValueLifetime::Execution),
            ("Work", TypeMetadata::Int, HostValueLifetime::Execution),
        ] {
            let type_ = TypeMetadata::External(NominalTypeMetadata {
                package: "storage".into(),
                module: "storage".into(),
                name: name.into(),
                arguments: vec![argument].into(),
            });
            assert_eq!(
                lifetimes.lifetime(&TypeMetadata::Tuple(vec![TypeMetadata::Bool, type_].into())),
                expected
            );
        }
        assert_eq!(
            Rust::expression(&HostValueLifetime::LoadedOwner),
            "data::host::HostValueLifetime::LoadedOwner"
        );
        assert_eq!(
            Rust::expression(&HostValueLifetime::Execution),
            "data::host::HostValueLifetime::Execution"
        );
    }

    #[test]
    fn seals_and_matches_custom_and_list_lifetimes() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Box(a) { Box(a) }
pub fn main() { #([Box(42)], [Box(fn(value: Int) { value + 1 })]) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let lifetimes = ValueLifetimes::new(
            &common.custom_types.definitions,
            &common.external_types.definitions,
        );
        assert!(lifetimes.matches(
            &common.list_types,
            &common.custom_types,
            &common.external_types
        ));
        assert_eq!(
            common
                .custom_types
                .types
                .iter()
                .map(|type_| type_.lifetime)
                .collect::<Vec<_>>(),
            [HostValueLifetime::LoadedOwner, HostValueLifetime::Execution]
        );
        assert_eq!(
            common.list_types.lifetimes.as_ref(),
            [HostValueLifetime::LoadedOwner, HostValueLifetime::Execution]
        );
        let mut lists = common.list_types.as_ref().clone();
        lists.lifetimes = vec![HostValueLifetime::Execution; lists.types.len()].into();
        assert!(!lifetimes.matches(&lists, &common.custom_types, &common.external_types));
    }
}
