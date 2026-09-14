use super::{ContractError, Function, HostedFunctionMetadata, Registration, Types};
use crate::host::HostTypeDescriptor;
use crate::plan::execution::host::HostTypeArgument;
use crate::plan::execution::type_::{
    CustomConstructorRefinement, ValueShapeDescriptor, ValueShapeId,
};

pub(super) fn admit(
    metadata: &HostedFunctionMetadata,
    registration: &Registration,
    declaration: &Function<'_>,
    types: &Types<'_>,
) -> Result<(), ContractError> {
    let parameters = registration.schema.parameters();
    if parameters.len() != declaration.parameter_shapes.len() {
        return Err(ContractError::Parameters);
    }
    for (parameter, shape) in parameters.iter().zip(declaration.parameter_shapes) {
        check(parameter, *shape, &metadata.type_arguments, types)?;
    }
    check(
        registration.schema.return_type(),
        declaration.return_,
        &metadata.type_arguments,
        types,
    )
}

fn check(
    descriptor: &HostTypeDescriptor,
    shape: ValueShapeId,
    parameters: &[HostTypeArgument],
    types: &Types<'_>,
) -> Result<(), ContractError> {
    use HostTypeDescriptor as Host;
    use ValueShapeDescriptor as Shape;
    if let Host::Parameter(index) = descriptor {
        let expected = parameters
            .get(*index)
            .ok_or(ContractError::TypeArguments)?
            .shape;
        return if types
            .equivalent(expected, shape)
            .map_err(ContractError::Type)?
        {
            Ok(())
        } else {
            Err(ContractError::Signature)
        };
    }
    match (descriptor, types.shape(shape).map_err(ContractError::Type)?) {
        (Host::Int, Shape::Int)
        | (Host::Float, Shape::Float)
        | (Host::String, Shape::String)
        | (Host::BitArray, Shape::BitArray)
        | (Host::UtfCodepoint, Shape::UtfCodepoint)
        | (Host::Bool, Shape::Bool)
        | (Host::Nil, Shape::Nil)
        | (Host::External { .. }, Shape::External(_)) => Ok(()),
        (Host::List(item), Shape::List(actual)) => check(item, *actual, parameters, types),
        (Host::Tuple(items), Shape::Tuple(actual)) => items_match(items, actual, parameters, types),
        (
            Host::Function { arguments, return_ } | Host::OpaqueFunction { arguments, return_ },
            Shape::Function {
                arguments: actual,
                return_: result,
            },
        ) => {
            items_match(arguments, actual, parameters, types)?;
            check(return_, *result, parameters, types)
        }
        (Host::Custom { arguments, .. }, Shape::Custom(id)) => {
            let actual = &types.shapes.custom_shapes[id.0];
            if actual.constructor != CustomConstructorRefinement::Any {
                return Err(ContractError::Signature);
            }
            items_match(arguments, &actual.arguments, parameters, types)
        }
        _ => Err(ContractError::Signature),
    }
}

fn items_match(
    expected: &[HostTypeDescriptor],
    actual: &[ValueShapeId],
    parameters: &[HostTypeArgument],
    types: &Types<'_>,
) -> Result<(), ContractError> {
    if expected.len() != actual.len() {
        return Err(ContractError::Signature);
    }
    for (expected, actual) in expected.iter().zip(actual) {
        check(expected, *actual, parameters, types)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ContractError, HostTypeArgument, Types, check};
    use crate::host::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema, HostSchemaType,
        HostTypeDescriptor,
    };
    use crate::plan::execution::prepared::admission::type_::TypeError;
    use crate::plan::execution::type_::{
        CustomConstructorRefinement, NominalTypeMetadata, TypeMetadata, ValueShapeDescriptor,
        ValueShapeId,
    };

    #[test]
    fn native_shape_matching_preserves_refinements_and_nested_signature_structure() {
        let module = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Choice { Present(Int) Missing }
fn widen(value: Choice) { value }
pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"a":utf8>>
  #(widen(Present(1)), Present(1), Missing, 1, 1.5, "text", <<1>>, codepoint,
    True, Nil, #(1, True), [1], fn(value: Int) { value + 1 })
}
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let shape = |metadata: &TypeMetadata| {
            common
                .value_shapes
                .shapes
                .iter()
                .enumerate()
                .find_map(|(index, _)| {
                    let id = ValueShapeId(index);
                    types
                        .matches(metadata, types.shape_type(id).unwrap())
                        .unwrap()
                        .then_some(id)
                })
                .unwrap()
        };
        for (descriptor, metadata) in [
            (HostTypeDescriptor::Int, TypeMetadata::Int),
            (HostTypeDescriptor::Float, TypeMetadata::Float),
            (HostTypeDescriptor::String, TypeMetadata::String),
            (HostTypeDescriptor::BitArray, TypeMetadata::BitArray),
            (HostTypeDescriptor::UtfCodepoint, TypeMetadata::UtfCodepoint),
            (HostTypeDescriptor::Bool, TypeMetadata::Bool),
            (HostTypeDescriptor::Nil, TypeMetadata::Nil),
            (
                HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int)),
                TypeMetadata::List(Box::new(TypeMetadata::Int).into()),
            ),
            (
                HostTypeDescriptor::Tuple(Box::new([
                    HostTypeDescriptor::Int,
                    HostTypeDescriptor::Bool,
                ])),
                TypeMetadata::Tuple(vec![TypeMetadata::Int, TypeMetadata::Bool].into()),
            ),
        ] {
            let actual = shape(&metadata);
            assert_eq!(check(&descriptor, actual, &[], &types), Ok(()));
            assert_eq!(
                check(&HostTypeDescriptor::Parameter(0), actual, &[], &types),
                Err(ContractError::TypeArguments)
            );
            assert_eq!(
                check(
                    &HostTypeDescriptor::Parameter(0),
                    actual,
                    &[HostTypeArgument {
                        type_: metadata,
                        shape: actual
                    }],
                    &types
                ),
                Ok(())
            );
        }
        let function = common
            .value_shapes
            .shapes
            .iter()
            .enumerate()
            .find_map(|(index, value)| {
                matches!(value, ValueShapeDescriptor::Function { .. })
                    .then_some(ValueShapeId(index))
            })
            .unwrap();
        for descriptor in [
            HostTypeDescriptor::Function {
                arguments: Box::new([HostTypeDescriptor::Int]),
                return_: Box::new(HostTypeDescriptor::Int),
            },
            HostTypeDescriptor::OpaqueFunction {
                arguments: Box::new([HostTypeDescriptor::Int]),
                return_: Box::new(HostTypeDescriptor::Int),
            },
        ] {
            assert_eq!(check(&descriptor, function, &[], &types), Ok(()));
        }
        for (arguments, return_) in [
            (vec![HostTypeDescriptor::Bool], HostTypeDescriptor::Int),
            (vec![HostTypeDescriptor::Int], HostTypeDescriptor::Bool),
            (vec![], HostTypeDescriptor::Int),
        ] {
            assert_eq!(
                check(
                    &HostTypeDescriptor::Function {
                        arguments: arguments.into_boxed_slice(),
                        return_: Box::new(return_),
                    },
                    function,
                    &[],
                    &types,
                ),
                Err(ContractError::Signature),
            );
        }
        assert_eq!(
            check(&HostTypeDescriptor::Int, ValueShapeId(999), &[], &types),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 })),
        );
        for (expected, actual) in [
            (ValueShapeId(999), shape(&TypeMetadata::Int)),
            (shape(&TypeMetadata::Int), ValueShapeId(999)),
        ] {
            assert_eq!(
                check(
                    &HostTypeDescriptor::Parameter(0),
                    actual,
                    &[HostTypeArgument {
                        type_: TypeMetadata::Int,
                        shape: expected
                    }],
                    &types,
                ),
                Err(ContractError::Type(TypeError::MissingShape { index: 999 })),
            );
        }
        assert_eq!(
            check(
                &HostTypeDescriptor::Tuple(Box::new([])),
                shape(&TypeMetadata::Tuple(
                    vec![TypeMetadata::Int, TypeMetadata::Bool].into()
                )),
                &[],
                &types
            ),
            Err(ContractError::Signature)
        );
        assert_eq!(
            check(
                &HostTypeDescriptor::Bool,
                shape(&TypeMetadata::Int),
                &[],
                &types
            ),
            Err(ContractError::Signature)
        );
        let (mut nominal, mut exact) = (None, None);
        for (index, value) in common.value_shapes.shapes.iter().enumerate() {
            if let ValueShapeDescriptor::Custom(id) = value {
                match types.custom_shape_descriptor(*id).unwrap().constructor {
                    CustomConstructorRefinement::Any => nominal = Some(ValueShapeId(index)),
                    CustomConstructorRefinement::Exact(0) => exact = Some(ValueShapeId(index)),
                    CustomConstructorRefinement::Exact(_) => {}
                }
            }
        }
        let nominal = nominal.unwrap();
        let exact = exact.unwrap();
        let metadata = TypeMetadata::Custom(NominalTypeMetadata {
            package: "geam".into(),
            module: "example".into(),
            name: "Choice".into(),
            arguments: vec![].into(),
        });
        let arguments = [HostTypeArgument {
            type_: metadata,
            shape: exact,
        }];
        assert_eq!(
            check(&HostTypeDescriptor::Parameter(0), exact, &arguments, &types),
            Ok(())
        );
        assert_eq!(
            check(
                &HostTypeDescriptor::Parameter(0),
                nominal,
                &arguments,
                &types
            ),
            Err(ContractError::Signature)
        );
        let descriptor = HostTypeDescriptor::Custom {
            schema: HostCustomTypeSchema::new(
                "geam",
                "example",
                "Choice",
                0,
                [
                    HostCustomConstructorSchema::new(
                        "Present",
                        [HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::Int,
                        )],
                    ),
                    HostCustomConstructorSchema::new("Missing", []),
                ],
            ),
            arguments: Box::new([]),
        };
        assert_eq!(check(&descriptor, nominal, &[], &types), Ok(()));
        assert_eq!(
            check(&descriptor, exact, &[], &types),
            Err(ContractError::Signature)
        );
    }
}
