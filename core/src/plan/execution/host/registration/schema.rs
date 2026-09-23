use super::same;
use crate::host::{HostCustomTypeSchema, HostExternalTypeSchema, HostSchemaType};
use crate::plan::Text;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalSchema {
    pub package: Text,
    pub module: Text,
    pub name: Text,
    pub parameter_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomSchema {
    pub package: Text,
    pub module: Text,
    pub name: Text,
    pub parameter_count: usize,
    pub constructors: Table<ConstructorSchema>,
    pub shared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorSchema {
    pub name: Text,
    pub fields: Table<FieldSchema>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldSchema {
    pub label: Option<Text>,
    pub type_: SchemaType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaType {
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
    Custom {
        package: Text,
        module: Text,
        name: Text,
        arguments: Table<Self>,
    },
    External {
        schema: ExternalSchema,
        arguments: Table<Self>,
    },
}

impl ExternalSchema {
    pub(super) fn from_schema(schema: &HostExternalTypeSchema) -> Self {
        Self {
            package: schema.package().clone().into(),
            module: schema.module().clone().into(),
            name: schema.name().clone().into(),
            parameter_count: schema.parameter_count(),
        }
    }

    pub(super) fn matches(&self, schema: &HostExternalTypeSchema) -> bool {
        self.package.as_str() == schema.package().as_str()
            && self.module.as_str() == schema.module().as_str()
            && self.name.as_str() == schema.name().as_str()
            && self.parameter_count == schema.parameter_count()
    }
}

impl CustomSchema {
    pub(super) fn from_schema(schema: &HostCustomTypeSchema) -> Self {
        Self {
            package: schema.package().clone().into(),
            module: schema.module().clone().into(),
            name: schema.name().clone().into(),
            parameter_count: schema.parameter_count(),
            shared: schema.requires_shared_access(),
            constructors: schema
                .constructors()
                .iter()
                .map(|constructor| ConstructorSchema {
                    name: constructor.name().clone().into(),
                    fields: constructor
                        .fields()
                        .iter()
                        .map(|field| FieldSchema {
                            label: field.label().cloned().map(Text::from),
                            type_: SchemaType::from_schema(field.type_()),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    pub(super) fn matches(&self, schema: &HostCustomTypeSchema) -> bool {
        self.package.as_str() == schema.package().as_str()
            && self.module.as_str() == schema.module().as_str()
            && self.name.as_str() == schema.name().as_str()
            && self.parameter_count == schema.parameter_count()
            && self.shared == schema.requires_shared_access()
            && same(&self.constructors, schema.constructors(), |left, right| {
                left.name.as_str() == right.name().as_str()
                    && same(&left.fields, right.fields(), |left, right| {
                        left.label.as_ref().map(Text::as_str)
                            == right.label().map(|label| label.as_str())
                            && left.type_.matches(right.type_())
                    })
            })
    }
}

impl SchemaType {
    fn from_schema(type_: &HostSchemaType) -> Self {
        match type_ {
            HostSchemaType::Parameter(index) => Self::Parameter(*index),
            HostSchemaType::Int => Self::Int,
            HostSchemaType::Float => Self::Float,
            HostSchemaType::String => Self::String,
            HostSchemaType::BitArray => Self::BitArray,
            HostSchemaType::UtfCodepoint => Self::UtfCodepoint,
            HostSchemaType::Bool => Self::Bool,
            HostSchemaType::Nil => Self::Nil,
            HostSchemaType::List(item) => Self::List(Box::new(Self::from_schema(item)).into()),
            HostSchemaType::Tuple(items) => {
                Self::Tuple(items.iter().map(Self::from_schema).collect())
            }
            HostSchemaType::Function { arguments, return_ } => Self::Function {
                arguments: arguments.iter().map(Self::from_schema).collect(),
                return_: Box::new(Self::from_schema(return_)).into(),
            },
            HostSchemaType::Custom {
                package,
                module,
                name,
                arguments,
            } => Self::Custom {
                package: package.clone().into(),
                module: module.clone().into(),
                name: name.clone().into(),
                arguments: arguments.iter().map(Self::from_schema).collect(),
            },
            HostSchemaType::External { schema, arguments } => Self::External {
                schema: ExternalSchema::from_schema(schema),
                arguments: arguments.iter().map(Self::from_schema).collect(),
            },
        }
    }

    fn matches(&self, type_: &HostSchemaType) -> bool {
        match (self, type_) {
            (Self::Parameter(left), HostSchemaType::Parameter(right)) => left == right,
            (Self::Int, HostSchemaType::Int) => true,
            (Self::Float, HostSchemaType::Float) => true,
            (Self::String, HostSchemaType::String) => true,
            (Self::BitArray, HostSchemaType::BitArray) => true,
            (Self::UtfCodepoint, HostSchemaType::UtfCodepoint) => true,
            (Self::Bool, HostSchemaType::Bool) => true,
            (Self::Nil, HostSchemaType::Nil) => true,
            (Self::List(left), HostSchemaType::List(right)) => left.matches(right),
            (Self::Tuple(left), HostSchemaType::Tuple(right)) => same(left, right, Self::matches),
            (
                Self::Function {
                    arguments: left,
                    return_: left_return,
                },
                HostSchemaType::Function {
                    arguments: right,
                    return_: right_return,
                },
            ) => same(left, right, Self::matches) && left_return.matches(right_return),
            (
                Self::Custom {
                    package,
                    module,
                    name,
                    arguments,
                },
                HostSchemaType::Custom {
                    package: actual_package,
                    module: actual_module,
                    name: actual_name,
                    arguments: actual_arguments,
                },
            ) => {
                package.as_str() == actual_package.as_str()
                    && module.as_str() == actual_module.as_str()
                    && name.as_str() == actual_name.as_str()
                    && same(arguments, actual_arguments, Self::matches)
            }
            (
                Self::External { schema, arguments },
                HostSchemaType::External {
                    schema: actual_schema,
                    arguments: actual_arguments,
                },
            ) => schema.matches(actual_schema) && same(arguments, actual_arguments, Self::matches),
            _ => false,
        }
    }
}

impl Emit for ExternalSchema {
    fn emit(&self, output: &mut Rust) {
        let Self {
            package,
            module,
            name,
            parameter_count,
        } = self;
        output.structure(
            "host::ExternalSchema",
            &[
                ("package", package),
                ("module", module),
                ("name", name),
                ("parameter_count", parameter_count),
            ],
        );
    }
}

impl Emit for CustomSchema {
    fn emit(&self, output: &mut Rust) {
        let Self {
            package,
            module,
            name,
            parameter_count,
            constructors,
            shared,
        } = self;
        output.structure(
            "host::CustomSchema",
            &[
                ("package", package),
                ("module", module),
                ("name", name),
                ("parameter_count", parameter_count),
                ("constructors", constructors),
                ("shared", shared),
            ],
        );
    }
}

impl Emit for ConstructorSchema {
    fn emit(&self, output: &mut Rust) {
        let Self { name, fields } = self;
        output.structure(
            "host::ConstructorSchema",
            &[("name", name), ("fields", fields)],
        );
    }
}

impl Emit for FieldSchema {
    fn emit(&self, output: &mut Rust) {
        let Self { label, type_ } = self;
        output.structure("host::FieldSchema", &[("label", label), ("type_", type_)]);
    }
}

impl Emit for SchemaType {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter(index) => output.call("host::SchemaType::Parameter", &[index]),
            Self::Int => output.path("host::SchemaType::Int"),
            Self::Float => output.path("host::SchemaType::Float"),
            Self::String => output.path("host::SchemaType::String"),
            Self::BitArray => output.path("host::SchemaType::BitArray"),
            Self::UtfCodepoint => output.path("host::SchemaType::UtfCodepoint"),
            Self::Bool => output.path("host::SchemaType::Bool"),
            Self::Nil => output.path("host::SchemaType::Nil"),
            Self::List(item) => output.call("host::SchemaType::List", &[item]),
            Self::Tuple(items) => output.call("host::SchemaType::Tuple", &[items]),
            Self::Function { arguments, return_ } => output.structure(
                "host::SchemaType::Function",
                &[("arguments", arguments), ("return_", return_)],
            ),
            Self::Custom {
                package,
                module,
                name,
                arguments,
            } => output.structure(
                "host::SchemaType::Custom",
                &[
                    ("package", package),
                    ("module", module),
                    ("name", name),
                    ("arguments", arguments),
                ],
            ),
            Self::External { schema, arguments } => output.structure(
                "host::SchemaType::External",
                &[("schema", schema), ("arguments", arguments)],
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConstructorSchema, CustomSchema, ExternalSchema, FieldSchema, SchemaType};
    use crate::host::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema,
        HostExternalTypeSchema, HostSchemaType,
    };
    use crate::plan::execution::prepared::rust::Rust;

    #[test]
    fn emits_and_matches_every_original_schema_type_without_erasing_identity() {
        use HostSchemaType as H;
        use SchemaType as S;
        let cases = [
            (
                H::Parameter(2),
                S::Parameter(2),
                "data::host::SchemaType::Parameter(2)",
            ),
            (H::Int, S::Int, "data::host::SchemaType::Int"),
            (H::Float, S::Float, "data::host::SchemaType::Float"),
            (H::String, S::String, "data::host::SchemaType::String"),
            (H::BitArray, S::BitArray, "data::host::SchemaType::BitArray"),
            (
                H::UtfCodepoint,
                S::UtfCodepoint,
                "data::host::SchemaType::UtfCodepoint",
            ),
            (H::Bool, S::Bool, "data::host::SchemaType::Bool"),
            (H::Nil, S::Nil, "data::host::SchemaType::Nil"),
            (
                H::List(Box::new(H::Int)),
                S::List(Box::new(S::Int).into()),
                "data::host::SchemaType::List(data::Storage::Static(&data::host::SchemaType::Int))",
            ),
            (
                H::Tuple(Box::new([H::Int, H::String])),
                S::Tuple(vec![S::Int, S::String].into()),
                r#"
data::host::SchemaType::Tuple(data::Storage::Static(&[
    data::host::SchemaType::Int,
    data::host::SchemaType::String,
]))"#
                    .trim_start_matches('\n'),
            ),
            (
                H::Function {
                    arguments: Box::new([H::Int]),
                    return_: Box::new(H::Bool),
                },
                S::Function {
                    arguments: vec![S::Int].into(),
                    return_: Box::new(S::Bool).into(),
                },
                r#"
data::host::SchemaType::Function {
    arguments: data::Storage::Static(&[
        data::host::SchemaType::Int,
    ]),
    return_: data::Storage::Static(&data::host::SchemaType::Bool),
}"#
                .trim_start_matches('\n'),
            ),
            (
                H::Custom {
                    package: "app".into(),
                    module: "types".into(),
                    name: "Box".into(),
                    arguments: Box::new([H::Int]),
                },
                S::Custom {
                    package: "app".into(),
                    module: "types".into(),
                    name: "Box".into(),
                    arguments: vec![S::Int].into(),
                },
                r#"
data::host::SchemaType::Custom {
    package: data::Text::Static("app"),
    module: data::Text::Static("types"),
    name: data::Text::Static("Box"),
    arguments: data::Storage::Static(&[
        data::host::SchemaType::Int,
    ]),
}"#
                .trim_start_matches('\n'),
            ),
            (
                H::External {
                    schema: HostExternalTypeSchema::new("app", "types", "Token", 1),
                    arguments: Box::new([H::Int]),
                },
                S::External {
                    schema: ExternalSchema {
                        package: "app".into(),
                        module: "types".into(),
                        name: "Token".into(),
                        parameter_count: 1,
                    },
                    arguments: vec![S::Int].into(),
                },
                r#"
data::host::SchemaType::External {
    schema: data::host::ExternalSchema {
        package: data::Text::Static("app"),
        module: data::Text::Static("types"),
        name: data::Text::Static("Token"),
        parameter_count: 1,
    },
    arguments: data::Storage::Static(&[
        data::host::SchemaType::Int,
    ]),
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (index, (source, expected, expression)) in cases.iter().enumerate() {
            assert_eq!(SchemaType::from_schema(source), *expected);
            assert_eq!(Rust::expression(expected), *expression);
            for (other, (source, _, _)) in cases.iter().enumerate() {
                assert_eq!(expected.matches(source), index == other);
            }
        }
    }

    #[test]
    fn nested_schema_matching_checks_arguments_results_and_nominal_coordinates() {
        use HostSchemaType as H;
        use SchemaType as S;
        assert!(!S::Parameter(1).matches(&H::Parameter(2)));
        assert!(!S::List(Box::new(S::Int).into()).matches(&H::List(Box::new(H::Bool))));
        assert!(!S::Tuple(vec![S::Int].into()).matches(&H::Tuple(Box::new([]))));
        assert!(!S::Tuple(vec![S::Int].into()).matches(&H::Tuple(Box::new([H::Bool]))));
        let function = S::Function {
            arguments: vec![S::Int].into(),
            return_: Box::new(S::Bool).into(),
        };
        for (arguments, return_) in [
            (vec![], H::Bool),
            (vec![H::String], H::Bool),
            (vec![H::Int], H::String),
        ] {
            assert!(!function.matches(&H::Function {
                arguments: arguments.into_boxed_slice(),
                return_: Box::new(return_),
            }));
        }
        let custom = S::Custom {
            package: "app".into(),
            module: "types".into(),
            name: "Box".into(),
            arguments: vec![S::Int].into(),
        };
        for (package, module, name, arguments) in [
            ("other", "types", "Box", vec![H::Int]),
            ("app", "other", "Box", vec![H::Int]),
            ("app", "types", "Other", vec![H::Int]),
            ("app", "types", "Box", vec![]),
            ("app", "types", "Box", vec![H::String]),
        ] {
            assert!(!custom.matches(&H::Custom {
                package: package.into(),
                module: module.into(),
                name: name.into(),
                arguments: arguments.into_boxed_slice(),
            }));
        }
        let external = S::External {
            schema: ExternalSchema {
                package: "app".into(),
                module: "types".into(),
                name: "Token".into(),
                parameter_count: 1,
            },
            arguments: vec![S::Int].into(),
        };
        for (package, module, name, parameter_count, arguments) in [
            ("other", "types", "Token", 1, vec![H::Int]),
            ("app", "other", "Token", 1, vec![H::Int]),
            ("app", "types", "Other", 1, vec![H::Int]),
            ("app", "types", "Token", 2, vec![H::Int]),
            ("app", "types", "Token", 1, vec![]),
            ("app", "types", "Token", 1, vec![H::String]),
        ] {
            assert!(!external.matches(&H::External {
                schema: HostExternalTypeSchema::new(package, module, name, parameter_count),
                arguments: arguments.into_boxed_slice(),
            }));
        }
    }

    #[test]
    fn custom_schema_preserves_constructor_order_labels_and_field_types() {
        let source = HostCustomTypeSchema::new(
            "app",
            "types",
            "Box",
            1,
            [
                HostCustomConstructorSchema::new(
                    "Box",
                    [
                        HostCustomFieldSchema::new(Some("value"), HostSchemaType::Parameter(0)),
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::String),
                    ],
                ),
                HostCustomConstructorSchema::new("Empty", []),
            ],
        );
        let expected = CustomSchema {
            shared: false,
            package: "app".into(),
            module: "types".into(),
            name: "Box".into(),
            parameter_count: 1,
            constructors: vec![
                ConstructorSchema {
                    name: "Box".into(),
                    fields: vec![
                        FieldSchema {
                            label: Some("value".into()),
                            type_: SchemaType::Parameter(0),
                        },
                        FieldSchema {
                            label: None,
                            type_: SchemaType::String,
                        },
                    ]
                    .into(),
                },
                ConstructorSchema {
                    name: "Empty".into(),
                    fields: vec![].into(),
                },
            ]
            .into(),
        };
        assert_eq!(CustomSchema::from_schema(&source), expected);
        assert!(expected.matches(&source));
        let shared = source.clone().with_shared_access(true);
        let required = CustomSchema::from_schema(&shared);
        assert!(required.matches(&shared));
        assert!(!required.matches(&source));
        assert!(!expected.matches(&shared));
        assert_eq!(required.clone(), required);
        let expression = r#"
data::host::CustomSchema {
    package: data::Text::Static("app"),
    module: data::Text::Static("types"),
    name: data::Text::Static("Box"),
    parameter_count: 1,
    constructors: data::Storage::Static(&[
        data::host::ConstructorSchema {
            name: data::Text::Static("Box"),
            fields: data::Storage::Static(&[
                data::host::FieldSchema {
                    label: Some(data::Text::Static("value")),
                    type_: data::host::SchemaType::Parameter(0),
                },
                data::host::FieldSchema {
                    label: None,
                    type_: data::host::SchemaType::String,
                },
            ]),
        },
        data::host::ConstructorSchema {
            name: data::Text::Static("Empty"),
            fields: data::Storage::Static(&[]),
        },
    ]),
    shared: false,
}"#
        .trim_start_matches('\n');
        assert_eq!(Rust::expression(&expected), expression);
        assert_eq!(
            Rust::expression(&required),
            expression.replace("shared: false", "shared: true")
        );
        for (package, module, name, count) in [
            ("other", "types", "Box", 1),
            ("app", "other", "Box", 1),
            ("app", "types", "Other", 1),
            ("app", "types", "Box", 2),
        ] {
            assert!(!expected.matches(&HostCustomTypeSchema::new(
                package,
                module,
                name,
                count,
                source.constructors().iter().cloned(),
            )));
        }
        for constructors in [
            vec![],
            source.constructors().iter().cloned().rev().collect(),
            vec![
                HostCustomConstructorSchema::new(
                    "Other",
                    source.constructors()[0].fields().iter().cloned(),
                ),
                source.constructors()[1].clone(),
            ],
            vec![
                HostCustomConstructorSchema::new("Box", []),
                source.constructors()[1].clone(),
            ],
        ] {
            assert!(!expected.matches(&HostCustomTypeSchema::new(
                "app",
                "types",
                "Box",
                1,
                constructors
            )));
        }
        for (label, type_) in [
            (None, HostSchemaType::Parameter(0)),
            (Some("other"), HostSchemaType::Parameter(0)),
            (Some("value"), HostSchemaType::Int),
        ] {
            assert!(!expected.matches(&HostCustomTypeSchema::new(
                "app",
                "types",
                "Box",
                1,
                [
                    HostCustomConstructorSchema::new(
                        "Box",
                        [
                            HostCustomFieldSchema::new(label, type_),
                            HostCustomFieldSchema::new(None::<&str>, HostSchemaType::String),
                        ]
                    ),
                    HostCustomConstructorSchema::new("Empty", []),
                ]
            )));
        }
    }
}
