use super::super::codegen::CompiledShape;
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
    Custom,
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
    Kernel,
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
        value_shape,
    )?;
    targets(
        &compiled.bools,
        &functions.value_returns.bool_functions,
        Family::Bool,
        |id| id.0,
        value_shape,
    )?;
    targets(
        &compiled.customs,
        &functions.value_returns.custom_functions,
        Family::Custom,
        |id| *id,
        custom_shape,
    )
}

fn targets<Id, Body: ExecutionFunctionBody, Entry: ExecutionFunctionEntry<Body>>(
    targets: &[CompiledFunction<Id>],
    functions: &[Entry],
    family: Family,
    index: impl Fn(&Id) -> usize,
    inspect: impl for<'body> Fn(
        &'body Body,
        &CompiledImplementation,
    ) -> Result<CompiledShape<'body, Body::Graph>, Reason>,
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
        let implementation = &target.implementation;
        let shape = inspect(entry.body(), implementation).map_err(error)?;
        if implementation.entry() != shape.start(shape.graph.entry()) {
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

fn value_shape<'body, Body: ExecutionFunctionBody>(
    body: &'body Body,
    implementation: &CompiledImplementation,
) -> Result<CompiledShape<'body, Body::Graph>, Reason> {
    match implementation {
        CompiledImplementation::Numeric(_) => CompiledShape::inspect(body.function_body()),
        CompiledImplementation::BitArray(_) => CompiledShape::inspect_bits(body.function_body()),
    }
    .ok_or(Reason::UnsupportedGraph)
}

fn custom_shape<'body, Body: ExecutionFunctionBody>(
    body: &'body Body,
    implementation: &CompiledImplementation,
) -> Result<CompiledShape<'body, Body::Graph>, Reason> {
    match implementation {
        CompiledImplementation::BitArray(_) => {
            CompiledShape::inspect_bits(body.function_body()).ok_or(Reason::UnsupportedGraph)
        }
        CompiledImplementation::Numeric(_) => Err(Reason::Kernel),
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{graph_body, owned_mut};
    use super::CompiledShape;
    use super::{CompiledError, Family, Reason, all, targets, value_shape};
    use crate::plan::execution::compiled::{
        BitArrayImplementation, CompiledFunction, CompiledFunctions, CompiledImplementation,
        NumericImplementation,
    };
    use crate::plan::execution::function::{
        ExecutionIntFunctionBody, IntFunctionId, ValueFunctionEntry,
    };
    use crate::plan::execution::graph::IntLocalId;
    use crate::plan::execution::host::{
        HostFunctionId, HostedExecutionProfile, HostedFunctionTarget,
    };
    use crate::runtime::compiled::tests::{metadata_bit_array, metadata_numeric};
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
    fn bit_array_and_custom_targets_require_their_exact_kernel_and_bit_prefixes() {
        use crate::plan::execution::function::FunctionBodyOwner;
        let execution = hosted_plan(
            r#"
pub type Outcome { Done(Int) Bad }
fn scan(input: BitArray, total: Int) -> Outcome {
  case input {
    <<value:8, rest:bits>> -> scan(rest, total + value)
    <<>> -> Done(total)
    _ -> Bad
  }
}
pub fn main() { scan(<<1, 2>>, 0) }
"#,
        );
        let functions = &execution.execution.program.functions;
        let entries = &functions.value_returns.custom_functions;
        let (index, shape) = entries
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| {
                CompiledShape::inspect_bits(FunctionBodyOwner::function_body(graph_body(entry)))
                    .map(|shape| (index, shape))
            })
            .next()
            .unwrap();
        let target = CompiledFunction {
            function: index,
            implementation: CompiledImplementation::BitArray(BitArrayImplementation {
                entry: shape.start(shape.graph.entry()),
                checkpoints: shape.checkpoints.clone().into(),
                run: metadata_bit_array,
            }),
        };

        let mut compiled = CompiledFunctions {
            ints: vec![].into(),
            bools: vec![].into(),
            customs: vec![target].into(),
        };
        assert_eq!(all(&compiled, functions), Ok(()));
        let mut points = shape.checkpoints.clone();
        points[0].bit_arrays += 1;
        owned_mut(&mut compiled.customs)[0].implementation =
            CompiledImplementation::BitArray(BitArrayImplementation {
                entry: shape.start(shape.graph.entry()),
                checkpoints: points.into(),
                run: metadata_bit_array,
            });
        assert_eq!(
            all(&compiled, functions),
            Err(CompiledError {
                family: Family::Custom,
                function: index,
                reason: Reason::Checkpoint(0)
            })
        );
        owned_mut(&mut compiled.customs)[0].implementation =
            CompiledImplementation::Numeric(NumericImplementation {
                entry: shape.start(shape.graph.entry()),
                checkpoints: shape.checkpoints.clone().into(),
                run: metadata_numeric,
            });

        assert_eq!(
            all(&compiled, functions),
            Err(CompiledError {
                family: Family::Custom,
                function: index,
                reason: Reason::Kernel
            })
        );
    }

    #[test]
    fn bit_array_int_targets_use_bit_shapes_and_reject_scalar_only_graphs() {
        let execution = hosted_plan(
            r#"
fn scan(input: BitArray, total: Int) -> Int {
  case input {
    <<value:8-signed, rest:bits>> -> scan(rest, total + value)
    _ -> total
  }
}
fn scalar(value: Int) -> Int {
  case value < 0 { True -> value - 1 False -> value + 1 }
}
pub fn main() { scalar(scan(<<1, 2>>, 0)) }
"#,
        );
        let functions = &execution.execution.program.functions;
        let entries = &functions.value_returns.int_functions;
        let (index, shape) = entries
            .iter()
            .enumerate()
            .find_map(|(index, entry)| {
                CompiledShape::inspect_bits(graph_body(entry)).map(|shape| (index, shape))
            })
            .unwrap();
        let mut compiled = CompiledFunctions {
            customs: vec![].into(),
            ints: vec![CompiledFunction {
                function: IntFunctionId(index),
                implementation: CompiledImplementation::BitArray(BitArrayImplementation {
                    entry: shape.start(shape.graph.entry()),
                    checkpoints: shape.checkpoints.clone().into(),
                    run: metadata_bit_array,
                }),
            }]
            .into(),
            bools: vec![].into(),
        };
        assert_eq!(all(&compiled, functions), Ok(()));

        let scalar = entries
            .iter()
            .enumerate()
            .find_map(|(index, entry)| CompiledShape::inspect(graph_body(entry)).map(|_| index))
            .unwrap();
        owned_mut(&mut compiled.ints)[0].function = IntFunctionId(scalar);
        assert_eq!(
            all(&compiled, functions),
            Err(CompiledError {
                family: Family::Int,
                function: scalar,
                reason: Reason::UnsupportedGraph,
            })
        );
    }

    #[test]
    fn admits_complete_links_and_rejects_target_and_checkpoint_mismatches() {
        let execution = hosted_plan(
            "fn choose(value: Int, flag: Bool) { case flag { True -> value + 1 False -> value - 1 } } pub fn main() { choose(7, True) }",
        );
        let functions = &execution.execution.program.functions;
        let shape =
            CompiledShape::inspect(graph_body(&functions.value_returns.int_functions[1])).unwrap();
        let entry = shape.start(shape.graph.entry());
        let make_target = || CompiledFunction {
            function: IntFunctionId(1),
            implementation: CompiledImplementation::Numeric(NumericImplementation {
                entry,
                checkpoints: shape.checkpoints.clone().into(),
                run: metadata_numeric,
            }),
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
            let mut implementation = NumericImplementation {
                entry,
                checkpoints: shape.checkpoints.clone().into(),
                run: metadata_numeric,
            };
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
            let target = CompiledFunction {
                function,
                implementation: CompiledImplementation::Numeric(implementation),
            };
            let mut entries = vec![target];
            if change == 1 {
                entries.push(make_target());
            }
            let compiled = CompiledFunctions {
                customs: vec![].into(),
                ints: entries.into(),
                bools: vec![].into(),
            };
            assert_eq!(
                all(&compiled, functions),
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
    fn bool_links_have_their_own_family_and_cannot_name_an_int_function() {
        use crate::plan::execution::function::BoolFunctionId;
        let plan = source_plan(
            "fn choose(value: Int, flag: Bool) { case value < 0 { True -> !flag False -> flag } } pub fn main() { choose(7, True) }",
        );
        let shape = CompiledShape::inspect(plan.bool_function(BoolFunctionId(1)).body()).unwrap();
        let mut compiled = CompiledFunctions {
            customs: vec![].into(),
            ints: vec![].into(),
            bools: vec![CompiledFunction {
                function: BoolFunctionId(1),
                implementation: CompiledImplementation::Numeric(NumericImplementation {
                    entry: shape.start(shape.graph.entry()),
                    checkpoints: shape.checkpoints.into(),
                    run: metadata_numeric,
                }),
            }]
            .into(),
        };

        assert_eq!(all(&compiled, &plan.program.functions), Ok(()));
        assert_eq!(
            targets(
                &compiled.bools,
                &plan.program.functions.value_returns.bool_functions[..0],
                Family::Bool,
                |id| id.0,
                value_shape,
            ),
            Err(CompiledError {
                family: Family::Bool,
                function: 1,
                reason: Reason::MissingFunction,
            })
        );
        owned_mut(&mut compiled.bools)[0].function = BoolFunctionId(999);
        assert_eq!(
            all(&compiled, &plan.program.functions),
            Err(CompiledError {
                family: Family::Bool,
                function: 999,
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
        let compiled = [CompiledFunction {
            function: IntFunctionId(0),
            implementation: CompiledImplementation::Numeric(NumericImplementation {
                entry: 0,
                checkpoints: vec![].into(),
                run: metadata_numeric,
            }),
        }];

        assert_eq!(
            targets(&compiled, &functions, Family::Int, |id| id.0, value_shape),
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
                    customs: vec![].into(),
                    ints: Vec::from(compiled).into(),
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
