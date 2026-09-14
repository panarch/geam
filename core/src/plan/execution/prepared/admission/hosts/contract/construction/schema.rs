use super::ContractError;
use crate::host::HostSchemaType;
use crate::plan::ValueType;
use crate::plan::execution::type_::custom::FieldRefinement;

pub(super) fn resolve(
    schema: &HostSchemaType,
    parameters: &[ValueType],
) -> Result<ValueType, ContractError> {
    Ok(match schema {
        HostSchemaType::Parameter(index) => parameters
            .get(*index)
            .cloned()
            .ok_or(ContractError::TypeArguments)?,
        HostSchemaType::Int => ValueType::Int,
        HostSchemaType::Float => ValueType::Float,
        HostSchemaType::String => ValueType::String,
        HostSchemaType::BitArray => ValueType::BitArray,
        HostSchemaType::UtfCodepoint => ValueType::UtfCodepoint,
        HostSchemaType::Bool => ValueType::Bool,
        HostSchemaType::Nil => ValueType::Nil,
        HostSchemaType::List(item) => ValueType::List(Box::new(resolve(item, parameters)?)),
        HostSchemaType::Tuple(items) => ValueType::Tuple(arguments(items, parameters)?),
        HostSchemaType::Function {
            arguments: inputs,
            return_,
        } => ValueType::Function(Box::new(crate::plan::FunctionType::new(
            arguments(inputs, parameters)?,
            resolve(return_, parameters)?,
        ))),
        HostSchemaType::Custom {
            package,
            module,
            name,
            arguments: inputs,
        } => ValueType::Custom(crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new(package.clone(), module.clone(), name.clone()),
            arguments(inputs, parameters)?,
        )),
        HostSchemaType::External {
            schema,
            arguments: inputs,
        } => ValueType::External(crate::plan::ExternalType::new(
            crate::plan::ExternalTypeName::new(
                schema.package().clone(),
                schema.module().clone(),
                schema.name().clone(),
            ),
            arguments(inputs, parameters)?,
        )),
    })
}

fn arguments(
    inputs: &[HostSchemaType],
    parameters: &[ValueType],
) -> Result<Vec<ValueType>, ContractError> {
    inputs
        .iter()
        .map(|input| resolve(input, parameters))
        .collect()
}

pub(super) fn refinement(stored: &FieldRefinement, source: &HostSchemaType) -> bool {
    match (stored, source) {
        (FieldRefinement::Argument(left), HostSchemaType::Parameter(right)) => left == right,
        (
            FieldRefinement::Value,
            HostSchemaType::Int
            | HostSchemaType::Float
            | HostSchemaType::String
            | HostSchemaType::BitArray
            | HostSchemaType::UtfCodepoint
            | HostSchemaType::Bool
            | HostSchemaType::Nil
            | HostSchemaType::External { .. },
        ) => true,
        (FieldRefinement::List(left), HostSchemaType::List(right)) => refinement(left, right),
        (FieldRefinement::Tuple(left), HostSchemaType::Tuple(right))
        | (
            FieldRefinement::Custom(left),
            HostSchemaType::Custom {
                arguments: right, ..
            },
        ) => refinements(left, right),
        (
            FieldRefinement::Function {
                arguments: left,
                return_: left_return,
            },
            HostSchemaType::Function {
                arguments: right,
                return_: right_return,
            },
        ) => refinements(left, right) && refinement(left_return, right_return),
        _ => false,
    }
}

fn refinements(left: &[FieldRefinement], right: &[HostSchemaType]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| refinement(left, right))
}

#[cfg(test)]
mod tests {
    use super::{ContractError, FieldRefinement, HostSchemaType, ValueType, refinement, resolve};
    use crate::host::HostExternalTypeSchema;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::{CustomType, CustomTypeName, ExternalType, ExternalTypeName, FunctionType};

    #[test]
    fn schema_resolution_preserves_each_scalar_and_substitutes_nested_parameters() {
        for (schema, expected) in [
            (HostSchemaType::Int, ValueType::Int),
            (HostSchemaType::Float, ValueType::Float),
            (HostSchemaType::String, ValueType::String),
            (HostSchemaType::BitArray, ValueType::BitArray),
            (HostSchemaType::UtfCodepoint, ValueType::UtfCodepoint),
            (HostSchemaType::Bool, ValueType::Bool),
            (HostSchemaType::Nil, ValueType::Nil),
        ] {
            assert_eq!(resolve(&schema, &[]), Ok(expected));
        }
        let external = HostExternalTypeSchema::new("app", "native", "Handle", 1);
        let cases = [
            (HostSchemaType::Parameter(0), ValueType::String),
            (
                HostSchemaType::List(Box::new(HostSchemaType::Parameter(0))),
                ValueType::List(Box::new(ValueType::String)),
            ),
            (
                HostSchemaType::Tuple(
                    vec![HostSchemaType::Parameter(0), HostSchemaType::Parameter(1)].into(),
                ),
                ValueType::Tuple(vec![ValueType::String, ValueType::Int]),
            ),
            (
                HostSchemaType::Function {
                    arguments: vec![HostSchemaType::Parameter(0)].into(),
                    return_: Box::new(HostSchemaType::Parameter(1)),
                },
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::String],
                    ValueType::Int,
                ))),
            ),
            (
                HostSchemaType::Custom {
                    package: "app".into(),
                    module: "model".into(),
                    name: "Box".into(),
                    arguments: vec![HostSchemaType::Parameter(0)].into(),
                },
                ValueType::Custom(CustomType::new(
                    CustomTypeName::new("app".into(), "model".into(), "Box".into()),
                    vec![ValueType::String],
                )),
            ),
            (
                HostSchemaType::External {
                    schema: external,
                    arguments: vec![HostSchemaType::Parameter(0)].into(),
                },
                ValueType::External(ExternalType::new(
                    ExternalTypeName::new("app".into(), "native".into(), "Handle".into()),
                    vec![ValueType::String],
                )),
            ),
        ];
        for (schema, expected) in cases {
            assert_eq!(
                resolve(&schema, &[ValueType::String, ValueType::Int]),
                Ok(expected)
            );
            assert_eq!(resolve(&schema, &[]), Err(ContractError::TypeArguments));
        }
        assert_eq!(
            resolve(
                &HostSchemaType::Function {
                    arguments: vec![HostSchemaType::Int].into(),
                    return_: Box::new(HostSchemaType::Parameter(1)),
                },
                &[ValueType::String]
            ),
            Err(ContractError::TypeArguments)
        );
    }

    #[test]
    fn refinements_keep_parameter_positions_and_composite_structure() {
        for schema in [
            HostSchemaType::Int,
            HostSchemaType::Float,
            HostSchemaType::String,
            HostSchemaType::BitArray,
            HostSchemaType::UtfCodepoint,
            HostSchemaType::Bool,
            HostSchemaType::Nil,
            HostSchemaType::External {
                schema: HostExternalTypeSchema::new("app", "native", "Handle", 1),
                arguments: vec![HostSchemaType::Parameter(0)].into(),
            },
        ] {
            assert!(refinement(&FieldRefinement::Value, &schema));
            assert!(!refinement(&FieldRefinement::Argument(0), &schema));
        }
        assert!(refinement(
            &FieldRefinement::Argument(0),
            &HostSchemaType::Parameter(0)
        ));
        assert!(!refinement(
            &FieldRefinement::Argument(1),
            &HostSchemaType::Parameter(0)
        ));
        assert!(!refinement(
            &FieldRefinement::Value,
            &HostSchemaType::Parameter(0)
        ));
        let cases = [
            (
                FieldRefinement::List(Node::Static(&FieldRefinement::Argument(0))),
                HostSchemaType::List(Box::new(HostSchemaType::Parameter(0))),
                HostSchemaType::List(Box::new(HostSchemaType::Parameter(1))),
            ),
            (
                FieldRefinement::Tuple(Table::Static(&[FieldRefinement::Argument(0)])),
                HostSchemaType::Tuple(vec![HostSchemaType::Parameter(0)].into()),
                HostSchemaType::Tuple(vec![HostSchemaType::Parameter(1)].into()),
            ),
            (
                FieldRefinement::Custom(Table::Static(&[FieldRefinement::Argument(0)])),
                HostSchemaType::Custom {
                    package: "app".into(),
                    module: "model".into(),
                    name: "Box".into(),
                    arguments: vec![HostSchemaType::Parameter(0)].into(),
                },
                HostSchemaType::Custom {
                    package: "app".into(),
                    module: "model".into(),
                    name: "Box".into(),
                    arguments: vec![HostSchemaType::Parameter(1)].into(),
                },
            ),
            (
                FieldRefinement::Function {
                    arguments: Table::Static(&[FieldRefinement::Argument(0)]),
                    return_: Node::Static(&FieldRefinement::Argument(1)),
                },
                HostSchemaType::Function {
                    arguments: vec![HostSchemaType::Parameter(0)].into(),
                    return_: Box::new(HostSchemaType::Parameter(1)),
                },
                HostSchemaType::Function {
                    arguments: vec![HostSchemaType::Parameter(1)].into(),
                    return_: Box::new(HostSchemaType::Parameter(1)),
                },
            ),
        ];
        for (stored, matching, changed) in cases {
            assert!(refinement(&stored, &matching));
            assert!(!refinement(&stored, &changed));
            assert!(!refinement(&stored, &HostSchemaType::Nil));
        }
        assert!(!refinement(
            &FieldRefinement::Tuple(Table::Static(&[])),
            &HostSchemaType::Tuple(vec![HostSchemaType::Int].into()),
        ));
        assert!(refinement(
            &FieldRefinement::Tuple(Table::Static(&[])),
            &HostSchemaType::Tuple(vec![].into()),
        ));
        assert!(!refinement(
            &FieldRefinement::Function {
                arguments: Table::Static(&[]),
                return_: Node::Static(&FieldRefinement::Argument(0)),
            },
            &HostSchemaType::Function {
                arguments: vec![].into(),
                return_: Box::new(HostSchemaType::Parameter(1)),
            },
        ));
    }
}
