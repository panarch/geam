use super::super::numeric::NumericShape;
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionProfile,
    FunctionTables,
};
use crate::plan::execution::numeric::{NumericFunction, NumericFunctions};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct NumericError {
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
    Entry,
    CheckpointCount,
    Checkpoint(usize),
}

pub(super) fn all<Profile: ExecutionProfile>(
    numeric: &NumericFunctions,
    functions: &FunctionTables<Profile>,
) -> Result<(), NumericError> {
    targets(
        &numeric.ints,
        &functions.value_returns.int_functions,
        Family::Int,
        |id| id.0,
    )?;
    targets(
        &numeric.bools,
        &functions.value_returns.bool_functions,
        Family::Bool,
        |id| id.0,
    )
}

fn targets<Id, Body: ExecutionFunctionBody, Entry: ExecutionFunctionEntry<Body>>(
    targets: &[NumericFunction<Id>],
    functions: &[Entry],
    family: Family,
    index: impl Fn(&Id) -> usize,
) -> Result<(), NumericError> {
    let mut previous = None;
    for target in targets {
        let function = index(&target.function);
        let error = |reason| NumericError {
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
        let shape = NumericShape::inspect(entry.body().function_body())
            .ok_or_else(|| error(Reason::UnsupportedGraph))?;
        let implementation = &target.implementation;
        if implementation.entry != shape.starts[shape.graph.entry().index()] {
            return Err(error(Reason::Entry));
        }
        if implementation.checkpoints.len() != shape.checkpoints.len() {
            return Err(error(Reason::CheckpointCount));
        }
        for (index, (actual, expected)) in implementation
            .checkpoints
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
    use super::NumericShape;
    use super::{Family, NumericError, Reason, all, targets};
    use crate::plan::execution::function::{
        ExecutionIntFunctionBody, IntFunctionId, ValueFunctionEntry,
    };
    use crate::plan::execution::graph::IntLocalId;
    use crate::plan::execution::host::{
        HostFunctionId, HostedExecutionProfile, HostedFunctionTarget,
    };
    use crate::plan::execution::numeric::{
        NumericFunction, NumericFunctions, NumericImplementation,
    };
    use crate::runtime::numeric::{NumericProgress, NumericValues};
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
            NumericShape::inspect(graph_body(&functions.value_returns.int_functions[1])).unwrap();
        let entry = shape.starts[shape.graph.entry().index()];
        let make_target = || NumericFunction {
            function: IntFunctionId(1),
            implementation: NumericImplementation {
                entry,
                checkpoints: shape.checkpoints.clone().into(),
                run: |point, _, _| NumericProgress::Yield(point),
            },
        };
        let mut values = NumericValues::default();
        let mut budget = 1;
        assert_eq!(
            (make_target().implementation.run)(entry, &mut values, &mut budget),
            NumericProgress::Yield(entry)
        );
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
            let mut target = make_target();
            let mut points = shape.checkpoints.clone();
            match change {
                0 | 1 => {}
                2 => target.function = IntFunctionId(999),
                3 => target.function = IntFunctionId(0),
                4 => target.implementation.entry = usize::MAX,
                5 => target.implementation.checkpoints = vec![].into(),
                _ => {
                    match change {
                        6 => points[0].block = crate::plan::execution::graph::BlockId(999),
                        7 => points[0].instruction += 1,
                        8 => points[0].ints += 1,
                        _ => points[0].bools += 1,
                    }
                    target.implementation.checkpoints = points.into();
                }
            }
            let mut entries = vec![target];
            if change == 1 {
                entries.push(make_target());
            }
            let numeric = NumericFunctions {
                ints: entries.into(),
                bools: vec![].into(),
            };
            assert_eq!(
                all(&numeric, functions),
                expected.map_or(Ok(()), |(function, reason)| Err(NumericError {
                    family: Family::Int,
                    function,
                    reason
                }))
            );
        }
        assert_eq!(all(&NumericFunctions::interpreted(), functions), Ok(()));
    }

    #[test]
    fn bool_links_have_their_own_family_and_cannot_name_an_int_function() {
        use crate::plan::execution::function::BoolFunctionId;
        let plan = source_plan(
            "fn choose(value: Int, flag: Bool) { case value < 0 { True -> !flag False -> flag } } pub fn main() { choose(7, True) }",
        );
        let shape = NumericShape::inspect(plan.bool_function(BoolFunctionId(1)).body()).unwrap();
        let numeric = NumericFunctions {
            ints: vec![].into(),
            bools: vec![NumericFunction {
                function: BoolFunctionId(1),
                implementation: NumericImplementation {
                    entry: shape.starts[shape.graph.entry().index()],
                    checkpoints: shape.checkpoints.into(),
                    run: |point, _, _| NumericProgress::Yield(point),
                },
            }]
            .into(),
        };
        let mut values = NumericValues::default();
        let mut budget = 1;
        assert_eq!(
            (numeric.bools[0].implementation.run)(0, &mut values, &mut budget),
            NumericProgress::Yield(0)
        );
        assert_eq!(all(&numeric, &plan.program.functions), Ok(()));
        assert_eq!(
            targets(
                &numeric.bools,
                &plan.program.functions.value_returns.bool_functions[..0],
                Family::Bool,
                |id| id.0
            ),
            Err(NumericError {
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
        let numeric = [NumericFunction {
            function: IntFunctionId(0),
            implementation: NumericImplementation {
                entry: 0,
                checkpoints: vec![].into(),
                run: |point, _, _| NumericProgress::Yield(point),
            },
        }];
        let mut values = NumericValues::default();
        let mut budget = 1;
        assert_eq!(
            (numeric[0].implementation.run)(0, &mut values, &mut budget),
            NumericProgress::Yield(0)
        );
        assert_eq!(
            targets(&numeric, &functions, Family::Int, |id| id.0),
            Err(NumericError {
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
                &NumericFunctions {
                    ints: Vec::from(numeric).into(),
                    bools: vec![].into(),
                },
                tables,
            ),
            Err(NumericError {
                family: Family::Int,
                function: 0,
                reason: Reason::HostFunction,
            }),
        );
    }
}
