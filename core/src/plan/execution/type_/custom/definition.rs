use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};
use crate::plan::execution::type_::{FunctionMetadata, NominalTypeMetadata, TypeMetadata};
use crate::plan::{self, Text};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomDefinition {
    pub package: Text,
    pub module: Text,
    pub name: Text,
    pub publicity: plan::CustomTypePublicity,
    pub opaque: bool,
    pub native_access: Option<crate::host::HostCustomAccess>,
    pub retention_lifetime: crate::host::HostValueLifetime,
    pub parameters: usize,
    pub constructors: Table<ConstructorDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorDefinition {
    pub name: Text,
    pub fields: Table<FieldDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDefinition {
    pub label: Option<Text>,
    pub type_: TypeMetadata,
}

pub(in crate::plan::execution) static RESULT: CustomDefinition = CustomDefinition {
    package: Text::Static(""),
    module: Text::Static("gleam"),
    name: Text::Static("Result"),
    publicity: plan::CustomTypePublicity::Public,
    opaque: false,
    native_access: None,
    retention_lifetime: crate::host::HostValueLifetime::LoadedOwner,
    parameters: 2,
    constructors: Table::Static(&[
        ConstructorDefinition {
            name: Text::Static("Ok"),
            fields: Table::Static(&[FieldDefinition {
                label: None,
                type_: TypeMetadata::Parameter(plan::TypeParameterId(0)),
            }]),
        },
        ConstructorDefinition {
            name: Text::Static("Error"),
            fields: Table::Static(&[FieldDefinition {
                label: None,
                type_: TypeMetadata::Parameter(plan::TypeParameterId(1)),
            }]),
        },
    ]),
};

impl CustomDefinition {
    pub(in crate::plan::execution) fn from_definition(
        definition: &plan::CustomTypeDefinition,
    ) -> Self {
        Self {
            package: definition.name().package().clone().into(),
            module: definition.name().module().clone().into(),
            name: definition.name().name().clone().into(),
            publicity: definition.publicity(),
            opaque: definition.is_opaque(),
            native_access: definition.native_access(),
            retention_lifetime: definition.retention_lifetime(),
            parameters: definition.parameters().len(),
            constructors: definition
                .constructors()
                .iter()
                .map(|constructor| ConstructorDefinition {
                    name: constructor.name().clone().into(),
                    fields: constructor
                        .fields()
                        .iter()
                        .map(|field| FieldDefinition {
                            label: field.label().cloned().map(Text::from),
                            type_: template(field.type_()),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    pub(in crate::plan::execution) fn identity(&self) -> (&str, &str, &str) {
        (&self.package, &self.module, &self.name)
    }

    pub(crate) fn native_visible(&self) -> bool {
        match self.native_access {
            Some(crate::host::HostCustomAccess::Retained) => false,
            Some(crate::host::HostCustomAccess::Shared) => true,
            _ => !self.opaque && self.publicity == plan::CustomTypePublicity::Public,
        }
    }
}

fn template(type_: &plan::CustomTypeTemplate) -> TypeMetadata {
    use plan::CustomTypeTemplate as T;
    match type_ {
        T::Int => TypeMetadata::Int,
        T::Float => TypeMetadata::Float,
        T::String => TypeMetadata::String,
        T::BitArray => TypeMetadata::BitArray,
        T::UtfCodepoint => TypeMetadata::UtfCodepoint,
        T::Bool => TypeMetadata::Bool,
        T::Nil => TypeMetadata::Nil,
        T::Parameter(id) => TypeMetadata::Parameter(plan::TypeParameterId(id.0)),
        T::Tuple(items) => TypeMetadata::Tuple(items.iter().map(template).collect()),
        T::List(item) => TypeMetadata::List(Node::Owned(Box::new(template(item)))),
        T::Function { arguments, return_ } => TypeMetadata::Function(FunctionMetadata {
            arguments: arguments.iter().map(template).collect(),
            return_: Node::Owned(Box::new(template(return_))),
        }),
        T::Custom { name, arguments } => TypeMetadata::Custom(NominalTypeMetadata {
            package: name.package().clone().into(),
            module: name.module().clone().into(),
            name: name.name().clone().into(),
            arguments: arguments.iter().map(template).collect(),
        }),
        T::External { name, arguments } => TypeMetadata::External(NominalTypeMetadata {
            package: name.package().clone().into(),
            module: name.module().clone().into(),
            name: name.name().clone().into(),
            arguments: arguments.iter().map(template).collect(),
        }),
    }
}

impl Emit for CustomDefinition {
    fn emit(&self, output: &mut Rust) {
        let Self {
            package,
            module,
            name,
            publicity,
            opaque,
            native_access,
            retention_lifetime,
            parameters,
            constructors,
        } = self;
        output.structure(
            "type_::CustomDefinition",
            &[
                ("package", package),
                ("module", module),
                ("name", name),
                ("publicity", publicity),
                ("opaque", opaque),
                ("native_access", native_access),
                ("retention_lifetime", retention_lifetime),
                ("parameters", parameters),
                ("constructors", constructors),
            ],
        );
    }
}

impl Emit for plan::CustomTypePublicity {
    fn emit(&self, output: &mut Rust) {
        output.path(match self {
            Self::Public => "type_::CustomTypePublicity::Public",
            Self::Private => "type_::CustomTypePublicity::Private",
            Self::Internal => "type_::CustomTypePublicity::Internal",
        });
    }
}

impl Emit for crate::host::HostCustomAccess {
    fn emit(&self, output: &mut Rust) {
        output.path(match self {
            Self::Declared => "host::HostCustomAccess::Declared",
            Self::Shared => "host::HostCustomAccess::Shared",
            Self::Retained => "host::HostCustomAccess::Retained",
        });
    }
}

impl Emit for ConstructorDefinition {
    fn emit(&self, output: &mut Rust) {
        let Self { name, fields } = self;
        output.structure(
            "type_::ConstructorDefinition",
            &[("name", name), ("fields", fields)],
        );
    }
}

impl Emit for FieldDefinition {
    fn emit(&self, output: &mut Rust) {
        let Self { label, type_ } = self;
        output.structure(
            "type_::FieldDefinition",
            &[("label", label), ("type_", type_)],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{ConstructorDefinition, CustomDefinition, FieldDefinition};
    use crate::plan::execution::{prepared::rust::Rust, storage::Table, type_::TypeMetadata};
    use crate::plan::{CustomTypePublicity, Text, TypeParameterId};

    #[test]
    fn nominal_retention_never_exposes_a_native_representation() {
        use crate::host::HostCustomAccess;

        for (publicity, opaque, native_access, expected) in [
            (CustomTypePublicity::Public, false, None, true),
            (CustomTypePublicity::Public, true, None, false),
            (CustomTypePublicity::Private, false, None, false),
            (CustomTypePublicity::Internal, false, None, false),
            (
                CustomTypePublicity::Public,
                false,
                Some(HostCustomAccess::Declared),
                true,
            ),
            (
                CustomTypePublicity::Private,
                false,
                Some(HostCustomAccess::Declared),
                false,
            ),
            (
                CustomTypePublicity::Public,
                true,
                Some(HostCustomAccess::Shared),
                true,
            ),
            (
                CustomTypePublicity::Private,
                true,
                Some(HostCustomAccess::Shared),
                true,
            ),
            (
                CustomTypePublicity::Public,
                false,
                Some(HostCustomAccess::Retained),
                false,
            ),
            (
                CustomTypePublicity::Private,
                true,
                Some(HostCustomAccess::Retained),
                false,
            ),
        ] {
            let definition = CustomDefinition {
                package: "producer".into(),
                module: "handles".into(),
                name: "Handle".into(),
                publicity,
                opaque,
                native_access,
                retention_lifetime: crate::HostValueLifetime::LoadedOwner,
                parameters: 0,
                constructors: Table::Static(&[]),
            };
            assert_eq!(definition.native_visible(), expected);
        }
    }

    #[test]
    fn emits_full_declarations_and_every_visibility() {
        let definition = CustomDefinition {
            retention_lifetime: crate::HostValueLifetime::LoadedOwner,
            package: Text::Static("app"),
            module: Text::Static("app/types"),
            name: Text::Static("Box"),
            publicity: CustomTypePublicity::Public,
            opaque: true,
            native_access: None,
            parameters: 1,
            constructors: Table::Static(&[ConstructorDefinition {
                name: Text::Static("Box"),
                fields: Table::Static(&[
                    FieldDefinition {
                        label: Some(Text::Static("value")),
                        type_: TypeMetadata::Parameter(TypeParameterId(0)),
                    },
                    FieldDefinition {
                        label: None,
                        type_: TypeMetadata::Int,
                    },
                ]),
            }]),
        };
        assert_eq!(definition.identity(), ("app", "app/types", "Box"));
        assert_eq!(
            Rust::expression(&definition),
            r#"
data::type_::CustomDefinition {
    package: data::Text::Static("app"),
    module: data::Text::Static("app/types"),
    name: data::Text::Static("Box"),
    publicity: data::type_::CustomTypePublicity::Public,
    opaque: true,
    native_access: None,
    retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
    parameters: 1,
    constructors: data::Storage::Static(&[
        data::type_::ConstructorDefinition {
            name: data::Text::Static("Box"),
            fields: data::Storage::Static(&[
                data::type_::FieldDefinition {
                    label: Some(data::Text::Static("value")),
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                },
                data::type_::FieldDefinition {
                    label: None,
                    type_: data::type_::TypeMetadata::Int,
                },
            ]),
        },
    ]),
}"#
            .trim_start_matches('\n')
        );
        for (publicity, expected) in [
            (
                CustomTypePublicity::Public,
                "data::type_::CustomTypePublicity::Public",
            ),
            (
                CustomTypePublicity::Private,
                "data::type_::CustomTypePublicity::Private",
            ),
            (
                CustomTypePublicity::Internal,
                "data::type_::CustomTypePublicity::Internal",
            ),
        ] {
            assert_eq!(Rust::expression(&publicity), expected);
        }
        for (access, expected) in [
            (
                crate::HostCustomAccess::Declared,
                "data::host::HostCustomAccess::Declared",
            ),
            (
                crate::HostCustomAccess::Shared,
                "data::host::HostCustomAccess::Shared",
            ),
            (
                crate::HostCustomAccess::Retained,
                "data::host::HostCustomAccess::Retained",
            ),
        ] {
            assert_eq!(Rust::expression(&access), expected);
        }
    }
}
