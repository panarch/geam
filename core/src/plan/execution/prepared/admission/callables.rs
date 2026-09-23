use super::input::{self, InputError};
use super::type_::{TypeError, Types};
use crate::plan::LibraryCallableSignature;
use crate::plan::execution::LibraryCallable;
use crate::plan::execution::type_::TypeMetadata;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CallableError {
    Type(TypeError),
    Input(InputError),
    Recursive,
    Count { expected: usize, actual: usize },
    Signature,
    UninhabitedArguments,
}

pub(super) fn admit(callables: &[LibraryCallable], types: &Types<'_>) -> Result<(), CallableError> {
    enum Visit<'a> {
        Enter(&'a LibraryCallable),
        Leave(*const LibraryCallable),
    }
    let mut active = HashSet::new();
    let mut complete = HashSet::new();
    let mut pending: Vec<_> = callables.iter().map(Visit::Enter).collect();
    while let Some(visit) = pending.pop() {
        match visit {
            Visit::Leave(id) => {
                active.remove(&id);
                complete.insert(id);
            }
            Visit::Enter(callable) => {
                let id = std::ptr::from_ref(callable);
                if complete.contains(&id) {
                    continue;
                }
                if !active.insert(id) {
                    return Err(CallableError::Recursive);
                }
                types
                    .function_type(&callable.type_)
                    .map_err(CallableError::Type)?;
                input::admit(&callable.inputs, types).map_err(CallableError::Input)?;
                pending.push(Visit::Leave(id));
                pending.extend(callable.callables.iter().map(Visit::Enter));
            }
        }
    }
    Ok(())
}

pub(super) fn mapping(
    actual: &[LibraryCallable],
    expected: &[LibraryCallableSignature],
    types: &Types<'_>,
) -> Result<(), CallableError> {
    let mut pending = vec![(actual, expected)];
    while let Some((actual, expected)) = pending.pop() {
        if actual.len() != expected.len() {
            return Err(CallableError::Count {
                expected: expected.len(),
                actual: actual.len(),
            });
        }
        for (actual, expected) in actual.iter().zip(expected) {
            let arguments = expected.type_.argument_types();
            if actual.type_.arguments.len() != arguments.len()
                || !actual
                    .type_
                    .arguments
                    .iter()
                    .zip(arguments)
                    .all(|(actual, expected)| {
                        types.metadata_matches_value(&TypeMetadata::from_public(expected), actual)
                    })
                || !types.metadata_matches_value(
                    &TypeMetadata::from_public(expected.type_.return_()),
                    &actual.type_.return_,
                )
            {
                return Err(CallableError::Signature);
            }
            for argument in arguments {
                if !types.matched_metadata_inhabited(&TypeMetadata::from_public(argument)) {
                    return Err(CallableError::UninhabitedArguments);
                }
            }
            input::mapping(
                &actual.inputs,
                &expected.input_variants,
                &expected.input_lists,
                &[],
                types,
            )
            .map_err(CallableError::Input)?;
            pending.push((&actual.callables, &expected.callables));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{CallableError, InputError, TypeError, TypeMetadata, Types, admit, mapping};
    use crate::plan::LibraryCallableSignature;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{
        CustomTypeId, CustomTypeTable, ExternalTypeTable, FunctionType, ListTypeTable,
        ValueShapeTable, ValueType,
    };
    use crate::plan::execution::{
        LibraryCallable, LibraryInputConstructions, LibraryListConstructions,
    };

    static LISTS: ListTypeTable = ListTypeTable {
        types: Table::Static(&[]),
        tuple_items: Table::Static(&[]),
        function_items: Table::Static(&[]),
    };
    static CUSTOMS: CustomTypeTable = CustomTypeTable {
        definitions: Table::Static(&[]),
        types: Table::Static(&[]),
    };
    static EXTERNALS: ExternalTypeTable = ExternalTypeTable {
        types: Table::Static(&[]),
    };
    static SHAPES: ValueShapeTable = ValueShapeTable {
        shapes: Table::Static(&[]),
        shape_types: Table::Static(&[]),
        custom_shapes: Table::Static(&[]),
    };
    const INPUTS: LibraryInputConstructions = LibraryInputConstructions {
        variants: Table::Static(&[]),
        lists: LibraryListConstructions {
            ints: Table::Static(&[]),
            floats: Table::Static(&[]),
            strings: Table::Static(&[]),
            bit_arrays: Table::Static(&[]),
            utf_codepoints: Table::Static(&[]),
            customs: Table::Static(&[]),
            externals: Table::Static(&[]),
            bools: Table::Static(&[]),
            nils: Table::Static(&[]),
            tuples: Table::Static(&[]),
            lists: Table::Static(&[]),
            functions: Table::Static(&[]),
        },
    };
    const INT_THUNK: FunctionType = FunctionType {
        arguments: Table::Static(&[]),
        return_: Node::Static(&ValueType::Int),
    };

    #[test]
    fn a_valid_symbolic_function_cannot_be_selected_as_an_invocable_callable() {
        let typed = crate::compile_typed_module(
            "example",
            "example.gleam",
            r#"
pub type Empty { Again(Empty) }
fn accept(value: Empty) { 42 }
pub fn main() { #(accept, 42) }
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
        let index = common
            .custom_types
            .types
            .iter()
            .position(|descriptor| descriptor.type_.name.as_str() == "Empty")
            .unwrap();
        let nominal = &common.custom_types.types[index].type_;
        let metadata = TypeMetadata::Custom(nominal.clone());
        let expected_argument = metadata.materialize();
        let actual = LibraryCallable {
            type_: FunctionType::new(vec![ValueType::Custom(CustomTypeId(index))], ValueType::Int),
            inputs: INPUTS,
            callables: Table::Static(&[]),
        };
        assert_eq!(admit(std::slice::from_ref(&actual), &types), Ok(()));
        let expected = LibraryCallableSignature {
            type_: crate::plan::FunctionType::new(
                vec![expected_argument],
                crate::plan::ValueType::Int,
            ),
            input_variants: vec![],
            input_lists: vec![],
            callables: vec![],
        };
        assert_eq!(
            mapping(&[actual], &[expected], &types),
            Err(CallableError::UninhabitedArguments)
        );
    }

    #[test]
    fn artifact_callable_graphs_accept_shared_children_and_reject_cycles_or_invalid_links() {
        static LEAF: [LibraryCallable; 1] = [LibraryCallable {
            type_: INT_THUNK,
            inputs: INPUTS,
            callables: Table::Static(&[]),
        }];
        const FACTORY: FunctionType = FunctionType {
            arguments: Table::Static(&[]),
            return_: Node::Static(&ValueType::Function(INT_THUNK)),
        };
        static SHARED: [LibraryCallable; 2] = [
            LibraryCallable {
                type_: FACTORY,
                inputs: INPUTS,
                callables: Table::Static(&LEAF),
            },
            LibraryCallable {
                type_: FACTORY,
                inputs: INPUTS,
                callables: Table::Static(&LEAF),
            },
        ];
        static CYCLE: [LibraryCallable; 1] = [LibraryCallable {
            type_: INT_THUNK,
            inputs: INPUTS,
            callables: Table::Static(&CYCLE),
        }];
        let types = Types::admit(&LISTS, &CUSTOMS, &EXTERNALS, &SHAPES).unwrap();
        assert_eq!(admit(&SHARED, &types), Ok(()));
        assert_eq!(admit(&CYCLE, &types), Err(CallableError::Recursive));
        let invalid = LibraryCallable {
            type_: FunctionType::new(vec![ValueType::Custom(CustomTypeId(0))], ValueType::Int),
            inputs: INPUTS,
            callables: Table::Static(&[]),
        };
        assert_eq!(
            admit(&[invalid], &types),
            Err(CallableError::Type(TypeError::MissingCustom { index: 0 }))
        );
        let mut invalid = LEAF[0].clone();
        invalid.inputs.lists.functions = vec![crate::plan::execution::type_::FunctionListTypeId {
            list_type: crate::plan::execution::type_::ListTypeId(0),
            item_type: crate::plan::execution::type_::list::FunctionItemTypeId(0),
        }]
        .into();
        assert_eq!(
            admit(&[invalid], &types),
            Err(CallableError::Input(InputError::Type(
                TypeError::MissingList { index: 0 }
            )))
        );
    }

    #[test]
    fn selection_matches_nested_signatures_and_their_input_construction_counts() {
        let types = Types::admit(&LISTS, &CUSTOMS, &EXTERNALS, &SHAPES).unwrap();
        let leaf = LibraryCallable {
            type_: INT_THUNK,
            inputs: INPUTS,
            callables: Table::Static(&[]),
        };
        let expected = LibraryCallableSignature {
            type_: crate::plan::FunctionType::new(vec![], crate::plan::ValueType::Int),
            input_variants: vec![],
            input_lists: vec![],
            callables: vec![],
        };
        assert_eq!(
            mapping(std::slice::from_ref(&leaf), &[], &types),
            Err(CallableError::Count {
                expected: 0,
                actual: 1
            })
        );
        let actual = LibraryCallable {
            type_: FunctionType::new(vec![], ValueType::Function(INT_THUNK)),
            callables: vec![leaf.clone()].into(),
            ..leaf.clone()
        };
        let nested = LibraryCallableSignature {
            type_: crate::plan::FunctionType::new(
                vec![],
                crate::plan::ValueType::Function(Box::new(expected.type_.clone())),
            ),
            callables: vec![expected.clone()],
            ..expected.clone()
        };
        assert_eq!(admit(std::slice::from_ref(&actual), &types), Ok(()));
        assert_eq!(mapping(&[actual], &[nested], &types), Ok(()));
        for signature in [
            crate::plan::FunctionType::new(
                vec![crate::plan::ValueType::Int],
                crate::plan::ValueType::Int,
            ),
            crate::plan::FunctionType::new(vec![], crate::plan::ValueType::Bool),
        ] {
            let changed = LibraryCallableSignature {
                type_: signature,
                ..expected.clone()
            };
            assert_eq!(
                mapping(std::slice::from_ref(&leaf), &[changed], &types),
                Err(CallableError::Signature)
            );
        }
        let actual = LibraryCallable {
            type_: FunctionType::new(vec![ValueType::Bool], ValueType::Int),
            ..leaf.clone()
        };
        let changed = LibraryCallableSignature {
            type_: crate::plan::FunctionType::new(
                vec![crate::plan::ValueType::Int],
                crate::plan::ValueType::Int,
            ),
            ..expected.clone()
        };
        assert_eq!(
            mapping(&[actual], &[changed], &types),
            Err(CallableError::Signature)
        );
        let changed = LibraryCallableSignature {
            input_variants: vec![crate::plan::LibraryVariant::new(
                crate::plan::StandardVariant::Result,
                vec![crate::plan::ValueType::Int, crate::plan::ValueType::Bool],
            )],
            ..expected
        };
        assert_eq!(
            mapping(&[leaf], &[changed], &types),
            Err(CallableError::Input(InputError::VariantCount {
                expected: 1,
                actual: 0
            }))
        );
    }
}
