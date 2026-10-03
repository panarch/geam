use super::super::codegen::CompiledShape;
use super::super::codegen::shape::KernelKind;
use crate::plan::execution::compiled::{
    CompiledFunction, CompiledFunctions, CompiledImplementation,
};
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionProfile,
    FunctionTables,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CompiledError {
    family: Family,
    function: usize,
    reason: Reason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Int,
    Bool,
}

#[derive(Debug, PartialEq, Eq)]
enum Reason {
    UnorderedTarget,
    MissingFunction,
    HostFunction,
    UnsupportedGraph,
    ImplementationKind,
    Entry,
    CheckpointCount,
    Checkpoint(usize),
}

pub(super) fn all<Profile: ExecutionProfile>(
    compiled: &CompiledFunctions,
    functions: &FunctionTables<Profile>,
) -> Result<(), CompiledError> {
    targets(
        &compiled.ints,
        &functions.value_returns.int_functions,
        Family::Int,
        |id| id.0,
    )?;
    targets(
        &compiled.bools,
        &functions.value_returns.bool_functions,
        Family::Bool,
        |id| id.0,
    )
}

fn targets<Id, Body: ExecutionFunctionBody, Entry: ExecutionFunctionEntry<Body>>(
    targets: &[CompiledFunction<Id>],
    functions: &[Entry],
    family: Family,
    index: impl Fn(&Id) -> usize,
) -> Result<(), CompiledError> {
    let mut previous = None;
    for target in targets {
        let function = index(&target.function);
        let error = |reason| CompiledError {
            family,
            function,
            reason,
        };
        if previous.is_some_and(|previous| previous >= function) {
            return Err(error(Reason::UnorderedTarget));
        }
        previous = Some(function);
        let entry = functions
            .get(function)
            .ok_or_else(|| error(Reason::MissingFunction))?;
        let ExecutionFunctionRef::Graph(entry) = entry.as_ref() else {
            return Err(error(Reason::HostFunction));
        };
        let shape = CompiledShape::inspect(entry.body().function_body())
            .ok_or_else(|| error(Reason::UnsupportedGraph))?;
        let implementation = &target.implementation;
        if !matches!(
            (implementation, shape.kind),
            (CompiledImplementation::Numeric(_), KernelKind::Numeric)
                | (CompiledImplementation::IntList(_), KernelKind::IntList)
        ) {
            return Err(error(Reason::ImplementationKind));
        }
        if implementation.entry() != shape.starts[shape.graph.entry().index()] {
            return Err(error(Reason::Entry));
        }
        if implementation.checkpoints().len() != shape.checkpoints.len() {
            return Err(error(Reason::CheckpointCount));
        }
        for (index, (actual, expected)) in implementation
            .checkpoints()
            .iter()
            .zip(&shape.checkpoints)
            .enumerate()
        {
            if actual != expected {
                return Err(error(Reason::Checkpoint(index)));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::tests::{graph_body, owned_mut};
    use super::{CompiledError, Family, Reason, all, targets};
    use super::{CompiledShape, KernelKind};
    use crate::plan::execution::compiled::{
        CompiledFunction, CompiledFunctions, CompiledImplementation, IntListImplementation,
        NumericImplementation,
    };
    use crate::plan::execution::function::{
        ExecutionIntFunctionBody, IntFunctionId, ValueFunctionEntry,
    };
    use crate::plan::execution::graph::IntLocalId;
    use crate::plan::execution::host::{
        HostFunctionId, HostedExecutionProfile, HostedFunctionTarget,
    };
    use crate::runtime::compiled::tests::{metadata_int_list, metadata_numeric};
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
    use std::marker::PhantomData;
    use std::sync::Arc;

    fn source_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap())
    }

    fn hosted_plan(source: &str) -> crate::HostedExecution<StatelessHostProfile> {
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
            .unwrap()
    }

    #[test]
    fn admits_complete_links_and_rejects_target_and_checkpoint_mismatches() {
        let execution = hosted_plan(
            "fn choose(value: Int, flag: Bool) { case flag { True -> value + 1 False -> value - 1 } } pub fn main() { choose(7, True) }",
        );
        let functions = &execution.execution.program.functions;
        let shape =
            CompiledShape::inspect(graph_body(&functions.value_returns.int_functions[1])).unwrap();
        let entry = shape.starts[shape.graph.entry().index()];
        let make_implementation = || NumericImplementation {
            entry,
            checkpoints: shape.checkpoints.clone().into(),
            run: metadata_numeric,
        };
        for (change, expected) in [
            (0, None),
            (1, Some((1, Reason::UnorderedTarget))),
            (2, Some((999, Reason::MissingFunction))),
            (3, Some((0, Reason::UnsupportedGraph))),
            (4, Some((1, Reason::Entry))),
            (5, Some((1, Reason::CheckpointCount))),
            (6, Some((1, Reason::Checkpoint(0)))),
            (7, Some((1, Reason::Checkpoint(0)))),
            (8, Some((1, Reason::Checkpoint(0)))),
            (9, Some((1, Reason::Checkpoint(0)))),
        ] {
            let mut function = IntFunctionId(1);
            let mut implementation = make_implementation();
            let mut points = shape.checkpoints.clone();
            match change {
                0 | 1 => {}
                2 => function = IntFunctionId(999),
                3 => function = IntFunctionId(0),
                4 => implementation.entry = usize::MAX,
                5 => implementation.checkpoints = vec![].into(),
                _ => {
                    match change {
                        6 => points[0].block = crate::plan::execution::graph::BlockId(999),
                        7 => points[0].instruction += 1,
                        8 => points[0].ints += 1,
                        _ => points[0].bools += 1,
                    }
                    implementation.checkpoints = points.into();
                }
            }
            let mut entries = vec![CompiledFunction {
                function,
                implementation: CompiledImplementation::Numeric(implementation),
            }];
            if change == 1 {
                entries.push(CompiledFunction {
                    function: IntFunctionId(1),
                    implementation: CompiledImplementation::Numeric(make_implementation()),
                });
            }
            let numeric = CompiledFunctions {
                ints: entries.into(),
                bools: vec![].into(),
            };
            assert_eq!(
                all(&numeric, functions),
                expected.map_or(Ok(()), |(function, reason)| Err(CompiledError {
                    family: Family::Int,
                    function,
                    reason
                }))
            );
        }
        assert_eq!(all(&CompiledFunctions::interpreted(), functions), Ok(()));
    }

    #[test]
    fn list_targets_require_the_matching_kernel_and_exact_list_prefix_in_both_return_families() {
        use crate::plan::execution::function::BoolFunctionId;

        let execution = hosted_plan(
            r#"
fn head(values: List(Int)) { case values { [value, ..] -> value _ -> 0 } }
fn same(left: List(Int), right: List(Int)) { left == right }
pub fn main() { #(head([1]), same([1], [1])) }
"#,
        );
        let functions = &execution.execution.program.functions;
        let integer =
            CompiledShape::inspect(graph_body(&functions.value_returns.int_functions[0])).unwrap();
        let boolean =
            CompiledShape::inspect(graph_body(&functions.value_returns.bool_functions[0])).unwrap();
        assert_eq!(integer.kind, KernelKind::IntList);
        assert_eq!(boolean.kind, KernelKind::IntList);
        for change in 0..4 {
            let mut points = integer.checkpoints.clone();
            if change == 2 {
                points[0].int_lists += 1;
            }
            let implementation = if change == 1 {
                CompiledImplementation::Numeric(NumericImplementation {
                    entry: integer.starts[integer.graph.entry().index()],
                    checkpoints: points.into(),
                    run: metadata_numeric,
                })
            } else {
                CompiledImplementation::IntList(IntListImplementation {
                    entry: integer.starts[integer.graph.entry().index()],
                    checkpoints: points.into(),
                    run: metadata_int_list,
                })
            };
            let boolean_implementation = if change == 3 {
                CompiledImplementation::Numeric(NumericImplementation {
                    entry: boolean.starts[boolean.graph.entry().index()],
                    checkpoints: boolean.checkpoints.clone().into(),
                    run: metadata_numeric,
                })
            } else {
                CompiledImplementation::IntList(IntListImplementation {
                    entry: boolean.starts[boolean.graph.entry().index()],
                    checkpoints: boolean.checkpoints.clone().into(),
                    run: metadata_int_list,
                })
            };
            let compiled = CompiledFunctions {
                ints: vec![CompiledFunction {
                    function: IntFunctionId(0),
                    implementation,
                }]
                .into(),
                bools: vec![CompiledFunction {
                    function: BoolFunctionId(0),
                    implementation: boolean_implementation,
                }]
                .into(),
            };
            let expected = match change {
                0 => Ok(()),
                1 => Err(CompiledError {
                    family: Family::Int,
                    function: 0,
                    reason: Reason::ImplementationKind,
                }),
                2 => Err(CompiledError {
                    family: Family::Int,
                    function: 0,
                    reason: Reason::Checkpoint(0),
                }),
                _ => Err(CompiledError {
                    family: Family::Bool,
                    function: 0,
                    reason: Reason::ImplementationKind,
                }),
            };
            assert_eq!(all(&compiled, functions), expected);
        }
    }

    #[test]
    fn bool_links_have_their_own_family_and_cannot_name_an_int_function() {
        use crate::plan::execution::function::BoolFunctionId;
        let plan = source_plan(
            "fn choose(value: Int, flag: Bool) { case value < 0 { True -> !flag False -> flag } } pub fn main() { choose(7, True) }",
        );
        let shape = CompiledShape::inspect(plan.bool_function(BoolFunctionId(1)).body()).unwrap();
        let numeric = CompiledFunctions {
            ints: vec![].into(),
            bools: vec![CompiledFunction {
                function: BoolFunctionId(1),
                implementation: CompiledImplementation::Numeric(NumericImplementation {
                    entry: shape.starts[shape.graph.entry().index()],
                    checkpoints: shape.checkpoints.into(),
                    run: metadata_numeric,
                }),
            }]
            .into(),
        };
        assert_eq!(all(&numeric, &plan.program.functions), Ok(()));
        assert_eq!(
            targets(
                &numeric.bools,
                &plan.program.functions.value_returns.bool_functions[..0],
                Family::Bool,
                |id| id.0
            ),
            Err(CompiledError {
                family: Family::Bool,
                function: 1,
                reason: Reason::MissingFunction,
            })
        );
    }

    #[test]
    fn a_generated_target_cannot_replace_a_host_entry() {
        // This is an invalid artifact link at this owner's boundary. Native
        // implementation bodies and their signatures remain the host's owner.
        let functions: [ValueFunctionEntry<
            ExecutionIntFunctionBody<HostedExecutionProfile>,
            HostedFunctionTarget<ExecutionIntFunctionBody<HostedExecutionProfile>>,
        >; 1] = [ValueFunctionEntry::host(HostedFunctionTarget::Value(
            HostFunctionId {
                index: 0,
                return_: IntLocalId(0),
                body: PhantomData,
            },
        ))];
        let numeric = [CompiledFunction {
            function: IntFunctionId(0),
            implementation: CompiledImplementation::Numeric(NumericImplementation {
                entry: 0,
                checkpoints: vec![].into(),
                run: metadata_numeric,
            }),
        }];
        assert_eq!(
            targets(&numeric, &functions, Family::Int, |id| id.0),
            Err(CompiledError {
                family: Family::Int,
                function: 0,
                reason: Reason::HostFunction
            })
        );

        let mut execution = hosted_plan("pub fn main() { 42 }");
        let program = Arc::get_mut(&mut execution.execution).unwrap();
        let tables = owned_mut(&mut program.program.functions);
        tables.value_returns.int_functions = Vec::from(functions).into();
        assert_eq!(
            all(
                &CompiledFunctions {
                    ints: Vec::from(numeric).into(),
                    bools: vec![].into(),
                },
                tables,
            ),
            Err(CompiledError {
                family: Family::Int,
                function: 0,
                reason: Reason::HostFunction,
            }),
        );
    }
}
