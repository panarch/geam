use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};

/// Relates a constructor field's refinements to its nominal type arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldRefinement {
    Value,
    Argument(usize),
    Tuple(Table<FieldRefinement>),
    List(Node<FieldRefinement>),
    Function {
        arguments: Table<FieldRefinement>,
        return_: Node<FieldRefinement>,
    },
    Custom(Table<FieldRefinement>),
}

impl Emit for FieldRefinement {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value => output.path("type_::FieldRefinement::Value"),
            Self::Argument(index) => output.call("type_::FieldRefinement::Argument", &[index]),
            Self::Tuple(elements) => output.call("type_::FieldRefinement::Tuple", &[elements]),
            Self::List(item) => output.call("type_::FieldRefinement::List", &[item]),
            Self::Function { arguments, return_ } => output.structure(
                "type_::FieldRefinement::Function",
                &[("arguments", arguments), ("return_", return_)],
            ),
            Self::Custom(arguments) => output.call("type_::FieldRefinement::Custom", &[arguments]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldRefinement, Node, Rust, Table};

    #[test]
    fn emits_each_field_relation_without_expanding_its_type_arguments() {
        for (refinement, expected) in [
            (
                FieldRefinement::Value,
                "data::type_::FieldRefinement::Value",
            ),
            (
                FieldRefinement::Argument(2),
                "data::type_::FieldRefinement::Argument(2)",
            ),
            (
                FieldRefinement::Tuple(Table::Static(&[
                    FieldRefinement::Value,
                    FieldRefinement::Argument(2),
                ])),
                r#"
data::type_::FieldRefinement::Tuple(data::Storage::Static(&[
    data::type_::FieldRefinement::Value,
    data::type_::FieldRefinement::Argument(2),
]))"#
                    .trim_start_matches('\n'),
            ),
            (
                FieldRefinement::List(Node::Static(&FieldRefinement::Argument(2))),
                "data::type_::FieldRefinement::List(data::Storage::Static(&data::type_::FieldRefinement::Argument(2)))",
            ),
            (
                FieldRefinement::Function {
                    arguments: Table::Static(&[FieldRefinement::Argument(2)]),
                    return_: Node::Static(&FieldRefinement::Value),
                },
                r#"
data::type_::FieldRefinement::Function {
    arguments: data::Storage::Static(&[
        data::type_::FieldRefinement::Argument(2),
    ]),
    return_: data::Storage::Static(&data::type_::FieldRefinement::Value),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FieldRefinement::Custom(Table::Static(&[FieldRefinement::Argument(2)])),
                r#"
data::type_::FieldRefinement::Custom(data::Storage::Static(&[
    data::type_::FieldRefinement::Argument(2),
]))"#
                    .trim_start_matches('\n'),
            ),
        ] {
            assert_eq!(Rust::expression(&refinement), expected);
        }
    }
}
