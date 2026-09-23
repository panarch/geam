use super::{CustomSchema, ExternalSchema, same};
use crate::host::HostTypeDescriptor;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationType {
    Parameter(usize),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List(Node<Self>),
    Tuple(Table<Self>),
    Function {
        arguments: Table<Self>,
        return_: Node<Self>,
    },
    OpaqueFunction {
        arguments: Table<Self>,
        return_: Node<Self>,
    },
    Custom {
        schema: CustomSchema,
        arguments: Table<Self>,
    },
    External {
        schema: ExternalSchema,
        arguments: Table<Self>,
    },
}

impl RegistrationType {
    pub(super) fn from_descriptor(descriptor: &HostTypeDescriptor) -> Self {
        match descriptor {
            HostTypeDescriptor::Parameter(index) => Self::Parameter(*index),
            HostTypeDescriptor::Int => Self::Int,
            HostTypeDescriptor::Float => Self::Float,
            HostTypeDescriptor::String => Self::String,
            HostTypeDescriptor::BitArray => Self::BitArray,
            HostTypeDescriptor::UtfCodepoint => Self::UtfCodepoint,
            HostTypeDescriptor::Bool => Self::Bool,
            HostTypeDescriptor::Nil => Self::Nil,
            HostTypeDescriptor::List(item) => {
                Self::List(Box::new(Self::from_descriptor(item)).into())
            }
            HostTypeDescriptor::Tuple(items) => {
                Self::Tuple(items.iter().map(Self::from_descriptor).collect())
            }
            HostTypeDescriptor::Function { arguments, return_ } => Self::Function {
                arguments: arguments.iter().map(Self::from_descriptor).collect(),
                return_: Box::new(Self::from_descriptor(return_)).into(),
            },
            HostTypeDescriptor::OpaqueFunction { arguments, return_ } => Self::OpaqueFunction {
                arguments: arguments.iter().map(Self::from_descriptor).collect(),
                return_: Box::new(Self::from_descriptor(return_)).into(),
            },
            HostTypeDescriptor::Custom { schema, arguments } => Self::Custom {
                schema: CustomSchema::from_schema(schema),
                arguments: arguments.iter().map(Self::from_descriptor).collect(),
            },
            HostTypeDescriptor::External { schema, arguments } => Self::External {
                schema: ExternalSchema::from_schema(schema),
                arguments: arguments.iter().map(Self::from_descriptor).collect(),
            },
        }
    }

    pub(super) fn matches(&self, descriptor: &HostTypeDescriptor) -> bool {
        match (self, descriptor) {
            (Self::Parameter(left), HostTypeDescriptor::Parameter(right)) => left == right,
            (Self::Int, HostTypeDescriptor::Int) => true,
            (Self::Float, HostTypeDescriptor::Float) => true,
            (Self::String, HostTypeDescriptor::String) => true,
            (Self::BitArray, HostTypeDescriptor::BitArray) => true,
            (Self::UtfCodepoint, HostTypeDescriptor::UtfCodepoint) => true,
            (Self::Bool, HostTypeDescriptor::Bool) => true,
            (Self::Nil, HostTypeDescriptor::Nil) => true,
            (Self::List(left), HostTypeDescriptor::List(right)) => left.matches(right),
            (Self::Tuple(left), HostTypeDescriptor::Tuple(right)) => {
                same(left, right, Self::matches)
            }
            (
                Self::Function {
                    arguments: left,
                    return_: left_return,
                },
                HostTypeDescriptor::Function {
                    arguments: right,
                    return_: right_return,
                },
            ) => same(left, right, Self::matches) && left_return.matches(right_return),
            (
                Self::OpaqueFunction {
                    arguments: left,
                    return_: left_return,
                },
                HostTypeDescriptor::OpaqueFunction {
                    arguments: right,
                    return_: right_return,
                },
            ) => same(left, right, Self::matches) && left_return.matches(right_return),
            (
                Self::Custom { schema, arguments },
                HostTypeDescriptor::Custom {
                    schema: actual_schema,
                    arguments: actual_arguments,
                },
            ) => schema.matches(actual_schema) && same(arguments, actual_arguments, Self::matches),
            (
                Self::External { schema, arguments },
                HostTypeDescriptor::External {
                    schema: actual_schema,
                    arguments: actual_arguments,
                },
            ) => schema.matches(actual_schema) && same(arguments, actual_arguments, Self::matches),
            _ => false,
        }
    }
}

impl Emit for RegistrationType {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter(index) => output.call("host::RegistrationType::Parameter", &[index]),
            Self::Int => output.path("host::RegistrationType::Int"),
            Self::Float => output.path("host::RegistrationType::Float"),
            Self::String => output.path("host::RegistrationType::String"),
            Self::BitArray => output.path("host::RegistrationType::BitArray"),
            Self::UtfCodepoint => output.path("host::RegistrationType::UtfCodepoint"),
            Self::Bool => output.path("host::RegistrationType::Bool"),
            Self::Nil => output.path("host::RegistrationType::Nil"),
            Self::List(item) => output.call("host::RegistrationType::List", &[item]),
            Self::Tuple(items) => output.call("host::RegistrationType::Tuple", &[items]),
            Self::Function { arguments, return_ } => output.structure(
                "host::RegistrationType::Function",
                &[("arguments", arguments), ("return_", return_)],
            ),
            Self::OpaqueFunction { arguments, return_ } => output.structure(
                "host::RegistrationType::OpaqueFunction",
                &[("arguments", arguments), ("return_", return_)],
            ),
            Self::Custom { schema, arguments } => output.structure(
                "host::RegistrationType::Custom",
                &[("schema", schema), ("arguments", arguments)],
            ),
            Self::External { schema, arguments } => output.structure(
                "host::RegistrationType::External",
                &[("schema", schema), ("arguments", arguments)],
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostTypeDescriptor, Node, RegistrationType, Rust};

    #[test]
    fn preserves_every_descriptor_family_and_does_not_erase_callback_permission() {
        use HostTypeDescriptor as D;
        let types = vec![
            D::Parameter(0),
            D::Parameter(1),
            D::Int,
            D::Float,
            D::String,
            D::BitArray,
            D::UtfCodepoint,
            D::Bool,
            D::Nil,
            D::List(Box::new(D::Int)),
            D::Tuple(Box::new([D::Int, D::String])),
            D::Function {
                arguments: Box::new([D::Int]),
                return_: Box::new(D::String),
            },
            D::OpaqueFunction {
                arguments: Box::new([D::Int]),
                return_: Box::new(D::String),
            },
            D::Custom {
                schema: crate::HostCustomTypeSchema::new("app", "types", "Empty", 1, []),
                arguments: Box::new([D::Int]),
            },
            D::External {
                schema: crate::HostExternalTypeSchema::new("app", "types", "Resource", 1),
                arguments: Box::new([D::Int]),
            },
        ];
        let frozen = types
            .iter()
            .map(RegistrationType::from_descriptor)
            .collect::<Vec<_>>();
        for (index, left) in frozen.iter().enumerate() {
            for (other_index, right) in types.iter().enumerate() {
                assert_eq!(
                    left.matches(right),
                    index == other_index,
                    "{index} {other_index}"
                );
            }
            assert_eq!(left.clone(), *left);
        }
        assert_eq!(types[11].value_type(), types[12].value_type());
        assert_ne!(frozen[11], frozen[12]);
        for (index, text) in [
            (0, "data::host::RegistrationType::Parameter(0)"),
            (2, "data::host::RegistrationType::Int"),
            (3, "data::host::RegistrationType::Float"),
            (4, "data::host::RegistrationType::String"),
            (5, "data::host::RegistrationType::BitArray"),
            (6, "data::host::RegistrationType::UtfCodepoint"),
            (7, "data::host::RegistrationType::Bool"),
            (8, "data::host::RegistrationType::Nil"),
            (
                9,
                "data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::Int))",
            ),
            (
                10,
                r#"
data::host::RegistrationType::Tuple(data::Storage::Static(&[
    data::host::RegistrationType::Int,
    data::host::RegistrationType::String,
]))"#
                    .trim_start_matches('\n'),
            ),
            (
                11,
                r#"
data::host::RegistrationType::Function {
    arguments: data::Storage::Static(&[
        data::host::RegistrationType::Int,
    ]),
    return_: data::Storage::Static(&data::host::RegistrationType::String),
}"#
                .trim_start_matches('\n'),
            ),
            (
                12,
                r#"
data::host::RegistrationType::OpaqueFunction {
    arguments: data::Storage::Static(&[
        data::host::RegistrationType::Int,
    ]),
    return_: data::Storage::Static(&data::host::RegistrationType::String),
}"#
                .trim_start_matches('\n'),
            ),
            (
                13,
                r#"
data::host::RegistrationType::Custom {
    schema: data::host::CustomSchema {
        package: data::Text::Static("app"),
        module: data::Text::Static("types"),
        name: data::Text::Static("Empty"),
        parameter_count: 1,
        constructors: data::Storage::Static(&[]),
        shared: false,
    },
    arguments: data::Storage::Static(&[
        data::host::RegistrationType::Int,
    ]),
}"#
                .trim_start_matches('\n'),
            ),
            (
                14,
                r#"
data::host::RegistrationType::External {
    schema: data::host::ExternalSchema {
        package: data::Text::Static("app"),
        module: data::Text::Static("types"),
        name: data::Text::Static("Resource"),
        parameter_count: 1,
    },
    arguments: data::Storage::Static(&[
        data::host::RegistrationType::Int,
    ]),
}"#
                .trim_start_matches('\n'),
            ),
        ] {
            assert_eq!(Rust::expression(&frozen[index]), text);
        }
        static CYCLE: RegistrationType = RegistrationType::List(Node::Static(&CYCLE));
        assert!(!CYCLE.matches(&D::List(Box::new(D::Int))));
        assert!(!frozen[10].matches(&D::Tuple(Box::new([D::Int]))));
    }

    #[test]
    fn nested_schema_and_nominal_changes_are_not_hidden_by_equal_outer_signatures() {
        use HostTypeDescriptor as D;
        let schema = crate::HostCustomTypeSchema::new(
            "app",
            "types",
            "Box",
            1,
            [crate::HostCustomConstructorSchema::new(
                "Box",
                [crate::HostCustomFieldSchema::new(
                    Some("value"),
                    crate::HostSchemaType::Parameter(0),
                )],
            )],
        );
        let original = D::Custom {
            schema: schema.clone(),
            arguments: Box::new([D::String]),
        };
        let frozen = RegistrationType::from_descriptor(&original);
        let changed = D::Custom {
            schema: crate::HostCustomTypeSchema::new(
                schema.package().clone(),
                schema.module().clone(),
                schema.name().clone(),
                1,
                [crate::HostCustomConstructorSchema::new(
                    "Box",
                    [crate::HostCustomFieldSchema::new(
                        Some("other"),
                        crate::HostSchemaType::Parameter(0),
                    )],
                )],
            ),
            arguments: Box::new([D::String]),
        };
        assert_eq!(original.value_type(), changed.value_type());
        assert!(!frozen.matches(&changed));
        let other_argument = D::Custom {
            schema: schema.clone(),
            arguments: Box::new([D::Int]),
        };
        assert!(!frozen.matches(&other_argument));
        let external = D::External {
            schema: crate::HostExternalTypeSchema::new("app", "types", "Handle", 1),
            arguments: Box::new([D::Int]),
        };
        let external_frozen = RegistrationType::from_descriptor(&external);
        for (package, module, name, count, arguments) in [
            ("other", "types", "Handle", 1, vec![D::Int]),
            ("app", "other", "Handle", 1, vec![D::Int]),
            ("app", "types", "Other", 1, vec![D::Int]),
            ("app", "types", "Handle", 2, vec![D::Int]),
            ("app", "types", "Handle", 1, vec![D::String]),
            ("app", "types", "Handle", 1, vec![]),
        ] {
            assert!(!external_frozen.matches(&D::External {
                schema: crate::HostExternalTypeSchema::new(package, module, name, count),
                arguments: arguments.into_boxed_slice(),
            }));
        }
    }
}
