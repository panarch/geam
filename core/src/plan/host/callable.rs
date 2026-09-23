use crate::host::{HostTypeDescriptor, RegisteredCallableConstruction};
use crate::plan::{FunctionInstantiation, HostFunctionTemplate};

pub(crate) fn instantiate_native_callable(
    definition: &HostFunctionTemplate,
    construction: &RegisteredCallableConstruction,
) -> Option<FunctionInstantiation> {
    if definition.parameters().len() != construction.arguments.len()
        || definition.captures().len() != construction.captures.len()
    {
        return None;
    }
    let mut arguments = vec![None; definition.scheme().parameters().len()];
    for (expected, actual) in definition
        .parameters()
        .iter()
        .zip(&construction.arguments)
        .chain(definition.captures().iter().zip(&construction.captures))
        .chain([(definition.return_type(), &construction.return_)])
    {
        if !bind_types(expected, actual, &mut arguments) {
            return None;
        }
    }
    // Registration requires contiguous parameters, all present in the signature
    // or captures. The signature owns the final substitution arity check.
    definition.signature().try_instantiate(
        arguments
            .into_iter()
            .flatten()
            .map(|type_| type_.value_shape())
            .collect(),
    )
}

// The definition and construction have different parameter scopes. Binding a
// definition parameter to a constructing function's parameter is not recursion.
fn bind_types(
    definition: &HostTypeDescriptor,
    construction: &HostTypeDescriptor,
    arguments: &mut [Option<HostTypeDescriptor>],
) -> bool {
    use HostTypeDescriptor as T;
    let mut pending = vec![(definition, construction)];
    while let Some((definition, construction)) = pending.pop() {
        let (expected, actual) = match (definition, construction) {
            (T::Parameter(index), actual) => {
                match &arguments[*index] {
                    Some(expected) if expected != actual => return false,
                    Some(_) => {}
                    None => arguments[*index] = Some(actual.clone()),
                }
                continue;
            }
            (T::List(expected), T::List(actual)) => {
                pending.push((expected, actual));
                continue;
            }
            (T::Tuple(expected), T::Tuple(actual)) => (expected, actual),
            (
                T::Function {
                    arguments: expected,
                    return_: expected_return,
                },
                T::Function {
                    arguments: actual,
                    return_: actual_return,
                },
            )
            | (
                T::OpaqueFunction {
                    arguments: expected,
                    return_: expected_return,
                },
                T::OpaqueFunction {
                    arguments: actual,
                    return_: actual_return,
                },
            ) => {
                pending.push((expected_return, actual_return));
                (expected, actual)
            }
            (
                T::Custom {
                    schema: expected_schema,
                    arguments: expected,
                },
                T::Custom {
                    schema: actual_schema,
                    arguments: actual,
                },
            ) if expected_schema == actual_schema => (expected, actual),
            (
                T::External {
                    schema: expected_schema,
                    arguments: expected,
                },
                T::External {
                    schema: actual_schema,
                    arguments: actual,
                },
            ) if expected_schema == actual_schema => (expected, actual),
            (expected, actual) if expected == actual => continue,
            _ => return false,
        };
        if expected.len() != actual.len() {
            return false;
        }
        pending.extend(expected.iter().zip(actual));
    }
    true
}

#[cfg(test)]
mod tests {
    use super::bind_types;
    use crate::host::{HostCustomTypeSchema, HostExternalTypeSchema, HostTypeDescriptor as T};

    #[test]
    fn recursive_declarations_bind_parameters_once_and_keep_nominal_and_callable_capabilities() {
        let custom = HostCustomTypeSchema::new("app", "records", "Box", 1, Vec::new());
        let external = HostExternalTypeSchema::new("app", "records", "Resource", 1);
        for (definition, actual) in [
            (
                T::List(Box::new(T::Parameter(0))),
                T::List(Box::new(T::Int)),
            ),
            (
                T::Tuple(Box::new([T::Parameter(0), T::Parameter(0)])),
                T::Tuple(Box::new([T::Int, T::Int])),
            ),
            (
                T::Function {
                    arguments: Box::new([T::Parameter(0)]),
                    return_: Box::new(T::Parameter(0)),
                },
                T::Function {
                    arguments: Box::new([T::Int]),
                    return_: Box::new(T::Int),
                },
            ),
            (
                T::OpaqueFunction {
                    arguments: Box::new([T::Parameter(0)]),
                    return_: Box::new(T::Parameter(0)),
                },
                T::OpaqueFunction {
                    arguments: Box::new([T::Int]),
                    return_: Box::new(T::Int),
                },
            ),
            (
                T::Custom {
                    schema: custom.clone(),
                    arguments: Box::new([T::Parameter(0)]),
                },
                T::Custom {
                    schema: custom.clone(),
                    arguments: Box::new([T::Int]),
                },
            ),
            (
                T::External {
                    schema: external.clone(),
                    arguments: Box::new([T::Parameter(0)]),
                },
                T::External {
                    schema: external.clone(),
                    arguments: Box::new([T::Int]),
                },
            ),
        ] {
            let mut arguments = [None];
            assert!(bind_types(&definition, &actual, &mut arguments));
            assert_eq!(arguments, [Some(T::Int)]);
        }
        for (definition, actual) in [
            (
                T::Tuple(Box::new([T::Parameter(0), T::Parameter(0)])),
                T::Tuple(Box::new([T::Int, T::Bool])),
            ),
            (
                T::Tuple(Box::new([T::Int])),
                T::Tuple(Box::new([T::Int, T::Int])),
            ),
            (
                T::Function {
                    arguments: Box::new([]),
                    return_: Box::new(T::Int),
                },
                T::OpaqueFunction {
                    arguments: Box::new([]),
                    return_: Box::new(T::Int),
                },
            ),
            (
                T::OpaqueFunction {
                    arguments: Box::new([]),
                    return_: Box::new(T::Int),
                },
                T::Function {
                    arguments: Box::new([]),
                    return_: Box::new(T::Int),
                },
            ),
            (
                T::Custom {
                    schema: custom,
                    arguments: Box::new([T::Int]),
                },
                T::Custom {
                    schema: HostCustomTypeSchema::new("other", "records", "Box", 1, Vec::new()),
                    arguments: Box::new([T::Int]),
                },
            ),
            (
                T::External {
                    schema: external,
                    arguments: Box::new([T::Int]),
                },
                T::External {
                    schema: HostExternalTypeSchema::new("app", "records", "Other", 1),
                    arguments: Box::new([T::Int]),
                },
            ),
        ] {
            assert!(!bind_types(&definition, &actual, &mut [None]));
        }
        assert!(bind_types(&T::Bool, &T::Bool, &mut []));
        assert!(!bind_types(&T::Bool, &T::Int, &mut []));
        // The caller's parameter zero and the body's parameter zero belong to
        // separate scopes, so an identity substitution is valid and finite.
        let mut arguments = [None];
        assert!(bind_types(
            &T::Parameter(0),
            &T::Parameter(0),
            &mut arguments
        ));
        assert_eq!(arguments, [Some(T::Parameter(0))]);
    }
}
