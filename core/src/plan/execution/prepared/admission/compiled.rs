mod calls;

use super::super::codegen::CompiledShape;
use super::super::codegen::shape::KernelKind;
use crate::plan::execution::compiled::{
    CompiledCallback, CompiledCallbackBodies, CompiledCallbackBody, CompiledFunction,
    CompiledFunctions, CompiledImplementation,
};
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionProfile,
    FunctionExit, FunctionTables,
};
use crate::plan::execution::type_::CustomTypeTable;

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
    IntList,
    IntCallback,
    BoolCallback,
    IntFunction,
    BoolFunction,
}

#[derive(Debug, PartialEq, Eq)]
enum Reason {
    UnorderedTarget,
    MissingFunction,
    TargetIdentity,
    HostFunction,
    UnsupportedGraph,
    ImplementationKind,
    Entry,
    CheckpointCount,
    Checkpoint(usize),
    Kernel,
    Calls,
    Returns,
    CallRoot,
    CallLocals,
    CallMapping,
    CallCaptures,
    CallReturns,
    CallTails,
}

pub(super) fn admit<'data, Profile: ExecutionProfile>(
    compiled: &'data CompiledFunctions,
    functions: &'data FunctionTables<Profile>,
    custom_types: &CustomTypeTable,
) -> Result<CompiledCallbackBodies<'data, Profile>, CompiledError> {
    calls::all(compiled, functions)?;
    targets(
        &compiled.ints,
        Family::Int,
        |id| id.0,
        |id| {
            functions
                .value_returns
                .int_functions
                .get(id.0)
                .map(ExecutionFunctionEntry::as_ref)
                .ok_or(Reason::MissingFunction)
        },
        |body, implementation| value_shape(body, implementation, custom_types),
    )?;
    targets(
        &compiled.bools,
        Family::Bool,
        |id| id.0,
        |id| {
            functions
                .value_returns
                .bool_functions
                .get(id.0)
                .map(ExecutionFunctionEntry::as_ref)
                .ok_or(Reason::MissingFunction)
        },
        |body, implementation| value_shape(body, implementation, custom_types),
    )?;
    targets(
        &compiled.customs,
        Family::Custom,
        |id| *id,
        |id| {
            functions
                .value_returns
                .custom_functions
                .get(*id)
                .map(ExecutionFunctionEntry::as_ref)
                .ok_or(Reason::MissingFunction)
        },
        custom_shape,
    )?;
    targets(
        &compiled.int_lists,
        Family::IntList,
        |id| id.index,
        |id| {
            let (expected, function) = functions
                .list_returns
                .int_list_functions
                .get(id.index)
                .ok_or(Reason::MissingFunction)?;
            if id != expected {
                return Err(Reason::TargetIdentity);
            }
            Ok(function.as_ref())
        },
        |body, implementation| match implementation {
            CompiledImplementation::BitArray(_)
            | CompiledImplementation::String(_)
            | CompiledImplementation::CustomLoop(_) => Err(Reason::Kernel),
            _ => value_shape(body, implementation, custom_types),
        },
    )?;
    let ints = callbacks(
        &compiled.callbacks.ints,
        Family::IntCallback,
        |id| id.0,
        |id| {
            functions
                .value_returns
                .int_functions
                .get(id.0)
                .map(ExecutionFunctionEntry::as_ref)
        },
        custom_types,
    )?;
    let bools = callbacks(
        &compiled.callbacks.bools,
        Family::BoolCallback,
        |id| id.0,
        |id| {
            functions
                .value_returns
                .bool_functions
                .get(id.0)
                .map(ExecutionFunctionEntry::as_ref)
        },
        custom_types,
    )?;
    Ok(CompiledCallbackBodies { ints, bools })
}

fn callbacks<
    'function,
    Id,
    Value,
    Local: PartialEq + 'static,
    Body: ExecutionFunctionBody<Return = Local> + 'function,
    Host: 'function,
>(
    callbacks: &'function [CompiledCallback<Id, Value, Local>],
    family: Family,
    index: impl Fn(&Id) -> usize,
    entry: impl Fn(&Id) -> Option<ExecutionFunctionRef<'function, Body, Host>>,
    types: &CustomTypeTable,
) -> Result<Vec<CompiledCallbackBody<'function, Body, Local>>, CompiledError> {
    let mut previous = None;
    let mut bodies = Vec::with_capacity(callbacks.len());
    for callback in callbacks {
        let function = index(&callback.function);
        let error = |reason| CompiledError {
            family,
            function,
            reason,
        };
        if previous.is_some_and(|previous| previous >= function) {
            return Err(error(Reason::UnorderedTarget));
        }
        previous = Some(function);
        let entry = entry(&callback.function).ok_or_else(|| error(Reason::MissingFunction))?;
        let ExecutionFunctionRef::Graph(entry) = entry else {
            return Err(error(Reason::HostFunction));
        };
        let body = entry.body().function_body();
        let shape = CompiledShape::inspect_callback(body, types)
            .ok_or_else(|| error(Reason::UnsupportedGraph))?;
        if callback.entry != shape.start(shape.graph.entry()) {
            return Err(error(Reason::Entry));
        }
        if callback.checkpoints.len() != shape.checkpoints.len() {
            return Err(error(Reason::CheckpointCount));
        }
        for (point, (actual, expected)) in callback
            .checkpoints
            .iter()
            .zip(&shape.checkpoints)
            .enumerate()
        {
            if actual != expected {
                return Err(error(Reason::Checkpoint(point)));
            }
        }
        if callback.returns.len() != body.exits.len() {
            return Err(error(Reason::Returns));
        }
        for (expected, actual) in body.exits.iter().zip(callback.returns.iter()) {
            if !matches!(expected, FunctionExit::Return(local) if local == actual) {
                return Err(error(Reason::Returns));
            }
        }
        bodies.push(CompiledCallbackBody {
            body: entry.body(),
            checkpoints: &callback.checkpoints,
            returns: &callback.returns,
        });
    }
    Ok(bodies)
}

fn targets<'function, Id, Body: ExecutionFunctionBody + 'function, HostTarget: 'function>(
    targets: &[CompiledFunction<Id>],
    family: Family,
    index: impl Fn(&Id) -> usize,
    entry: impl Fn(&Id) -> Result<ExecutionFunctionRef<'function, Body, HostTarget>, Reason>,
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
        let ExecutionFunctionRef::Graph(entry) = entry(&target.function).map_err(&error)? else {
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
    custom_types: &CustomTypeTable,
) -> Result<CompiledShape<'body, Body::Graph>, Reason> {
    let expected = match implementation {
        CompiledImplementation::CustomLoop(loop_) => {
            let shape = CompiledShape::inspect_custom_loop(body.function_body(), custom_types)
                .ok_or(Reason::UnsupportedGraph)?;
            if loop_.calls.as_ref() != shape.loop_calls().as_slice() {
                return Err(Reason::Calls);
            }
            return Ok(shape);
        }
        CompiledImplementation::Numeric(_) => KernelKind::Numeric,
        CompiledImplementation::IntList(_) => KernelKind::IntList,
        CompiledImplementation::String(_) => KernelKind::String,
        CompiledImplementation::BitArray(_) => {
            return CompiledShape::inspect_bits(body.function_body())
                .ok_or(Reason::UnsupportedGraph);
        }
        CompiledImplementation::FunctionCalls(_) => return Err(Reason::Kernel),
    };
    let shape = CompiledShape::inspect(body.function_body()).ok_or(Reason::UnsupportedGraph)?;
    if shape.kind == expected {
        Ok(shape)
    } else {
        Err(Reason::ImplementationKind)
    }
}

fn custom_shape<'body, Body: ExecutionFunctionBody>(
    body: &'body Body,
    implementation: &CompiledImplementation,
) -> Result<CompiledShape<'body, Body::Graph>, Reason> {
    match implementation {
        CompiledImplementation::BitArray(_) => {
            CompiledShape::inspect_bits(body.function_body()).ok_or(Reason::UnsupportedGraph)
        }
        _ => Err(Reason::Kernel),
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{graph_body, owned_mut};
    use super::{CompiledError, Family, Reason, admit};
    use super::{CompiledShape, KernelKind};
    use crate::plan::execution::compiled::{
        BitArrayImplementation, CompiledCallback, CompiledCallbacks, CompiledFunction,
        CompiledFunctions, CompiledImplementation, CustomLoopImplementation, IntListImplementation,
        NumericImplementation, StringImplementation,
    };
    use crate::plan::execution::function::{
        ExecutionIntFunctionBody, IntFunctionId, ValueFunctionEntry,
    };
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId, ParamLocal};
    use crate::plan::execution::host::{
        HostFunctionId, HostedExecutionProfile, HostedFunctionTarget,
    };
    use crate::plan::execution::type_::CustomTypeTable;
    use crate::runtime::compiled::tests::{
        metadata_bit_array, metadata_custom_loop, metadata_int_list, metadata_numeric,
        metadata_string,
    };
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
    use std::marker::PhantomData;
    use std::sync::Arc;

    // These rejection examples compare the validity contract. The successful
    // callback example below also checks the resolved original body views.
    fn all<Profile: super::ExecutionProfile>(
        compiled: &CompiledFunctions,
        functions: &super::FunctionTables<Profile>,
        custom_types: &CustomTypeTable,
    ) -> Result<(), CompiledError> {
        admit(compiled, functions, custom_types).map(|_| ())
    }

    fn source_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap())
    }

    #[test]
    fn string_targets_require_the_string_kind_and_exact_completed_string_prefix() {
        let execution = hosted_plan(
            r#"
fn walk(text: String, total: Int) {
  case text { "λ" <> rest -> walk(rest, total + 1) _ -> total }
}
pub fn main() { walk("λλ", 3) }
"#,
        );
        let functions = &execution.execution.program.functions;
        let shape =
            CompiledShape::inspect(graph_body(&functions.value_returns.int_functions[1])).unwrap();
        assert_eq!(shape.kind, KernelKind::String);
        let entry = shape.start(shape.graph.entry());
        for (change, expected) in [
            (0, Ok(())),
            (1, Err(Reason::ImplementationKind)),
            (2, Err(Reason::Checkpoint(1))),
            (3, Err(Reason::Checkpoint(0))),
            (4, Err(Reason::Entry)),
            (5, Err(Reason::CheckpointCount)),
        ] {
            let mut points = shape.checkpoints.clone();
            match change {
                2 => points[1].strings += 1,
                3 => points[0].strings = 0,
                5 => {
                    points.pop();
                }
                _ => {}
            }
            let implementation = if change == 1 {
                CompiledImplementation::Numeric(NumericImplementation {
                    entry,
                    checkpoints: points.into(),
                    run: metadata_numeric,
                })
            } else {
                CompiledImplementation::String(StringImplementation {
                    entry: if change == 4 { usize::MAX } else { entry },
                    checkpoints: points.into(),
                    run: metadata_string,
                })
            };
            let compiled = CompiledFunctions {
                ints: vec![CompiledFunction {
                    function: IntFunctionId(1),
                    implementation,
                }]
                .into(),
                ..CompiledFunctions::interpreted()
            };
            assert_eq!(
                all(
                    &compiled,
                    functions,
                    &execution.execution.program.common.custom_types
                ),
                expected.map_err(|reason| CompiledError {
                    family: Family::Int,
                    function: 1,
                    reason
                })
            );
        }
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
    fn list_return_targets_admit_the_full_identity_and_reject_wrong_types_and_positions() {
        use crate::plan::execution::type_::ListTypeId;
        let plan = source_plan("pub fn main() -> List(Int) { [7, -9] }");
        let functions = &plan.program.functions;
        let (id, body) = &functions.list_returns.int_list_functions[0];
        let shape = CompiledShape::inspect(body.body()).unwrap();
        for (change, expected) in [
            (0, None),
            (1, Some(Reason::TargetIdentity)),
            (2, Some(Reason::MissingFunction)),
            (3, Some(Reason::UnorderedTarget)),
            (4, Some(Reason::ImplementationKind)),
            (5, Some(Reason::Entry)),
            (6, Some(Reason::CheckpointCount)),
            (7, Some(Reason::Checkpoint(3))),
            (8, Some(Reason::Kernel)),
        ] {
            let mut function = *id;
            let mut points = shape.checkpoints.clone();
            if change == 1 {
                function.type_id.list_type = ListTypeId(999);
            }
            if change == 2 {
                function.index = 999;
            }
            if change == 6 {
                points.pop();
            }
            if change == 7 {
                points[3].int_lists += 1;
            }
            let make = || CompiledFunction {
                function,
                implementation: if change == 8 {
                    CompiledImplementation::BitArray(BitArrayImplementation {
                        entry: 0,
                        checkpoints: points.clone().into(),
                        run: metadata_bit_array,
                    })
                } else if change == 4 {
                    CompiledImplementation::Numeric(NumericImplementation {
                        entry: 0,
                        checkpoints: points.clone().into(),
                        run: metadata_numeric,
                    })
                } else {
                    CompiledImplementation::IntList(IntListImplementation {
                        entry: if change == 5 { 999 } else { 0 },
                        checkpoints: points.clone().into(),
                        run: metadata_int_list,
                    })
                },
            };
            let mut entries = vec![make()];
            if change == 3 {
                entries.push(make());
            }
            let compiled = CompiledFunctions {
                function_calls: vec![].into(),
                ints: vec![].into(),
                bools: vec![].into(),
                customs: vec![].into(),
                int_lists: entries.into(),
                callbacks: CompiledCallbacks::interpreted(),
            };
            assert_eq!(
                all(&compiled, functions, &plan.program.common.custom_types),
                expected.map_or(Ok(()), |reason| Err(CompiledError {
                    family: Family::IntList,
                    function: function.index,
                    reason,
                }))
            );
        }
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
            function_calls: vec![].into(),
            ints: vec![].into(),
            bools: vec![].into(),
            customs: vec![target].into(),
            int_lists: vec![].into(),
            callbacks: CompiledCallbacks::interpreted(),
        };
        assert_eq!(
            all(
                &compiled,
                functions,
                &execution.execution.program.common.custom_types
            ),
            Ok(())
        );
        let mut points = shape.checkpoints.clone();
        points[0].bit_arrays += 1;
        owned_mut(&mut compiled.customs)[0].implementation =
            CompiledImplementation::BitArray(BitArrayImplementation {
                entry: shape.start(shape.graph.entry()),
                checkpoints: points.into(),
                run: metadata_bit_array,
            });
        assert_eq!(
            all(
                &compiled,
                functions,
                &execution.execution.program.common.custom_types
            ),
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
            all(
                &compiled,
                functions,
                &execution.execution.program.common.custom_types
            ),
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
            function_calls: vec![].into(),
            customs: vec![].into(),
            int_lists: vec![].into(),
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
            callbacks: CompiledCallbacks::interpreted(),
        };
        assert_eq!(
            all(
                &compiled,
                functions,
                &execution.execution.program.common.custom_types
            ),
            Ok(())
        );

        let scalar = entries
            .iter()
            .enumerate()
            .find_map(|(index, entry)| CompiledShape::inspect(graph_body(entry)).map(|_| index))
            .unwrap();
        owned_mut(&mut compiled.ints)[0].function = IntFunctionId(scalar);
        assert_eq!(
            all(
                &compiled,
                functions,
                &execution.execution.program.common.custom_types
            ),
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
                function_calls: vec![].into(),
                customs: vec![].into(),
                int_lists: vec![].into(),
                ints: entries.into(),
                bools: vec![].into(),
                callbacks: CompiledCallbacks::interpreted(),
            };
            assert_eq!(
                all(
                    &compiled,
                    functions,
                    &execution.execution.program.common.custom_types
                ),
                expected.map_or(Ok(()), |(function, reason)| Err(CompiledError {
                    family: Family::Int,
                    function,
                    reason
                }))
            );
        }
        assert_eq!(
            all(
                &CompiledFunctions::interpreted(),
                functions,
                &execution.execution.program.common.custom_types
            ),
            Ok(())
        );
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
                    entry: integer.start(integer.graph.entry()),
                    checkpoints: points.into(),
                    run: metadata_numeric,
                })
            } else {
                CompiledImplementation::IntList(IntListImplementation {
                    entry: integer.start(integer.graph.entry()),
                    checkpoints: points.into(),
                    run: metadata_int_list,
                })
            };
            let boolean_implementation = if change == 3 {
                CompiledImplementation::Numeric(NumericImplementation {
                    entry: boolean.start(boolean.graph.entry()),
                    checkpoints: boolean.checkpoints.clone().into(),
                    run: metadata_numeric,
                })
            } else {
                CompiledImplementation::IntList(IntListImplementation {
                    entry: boolean.start(boolean.graph.entry()),
                    checkpoints: boolean.checkpoints.clone().into(),
                    run: metadata_int_list,
                })
            };
            let compiled = CompiledFunctions {
                function_calls: vec![].into(),
                customs: vec![].into(),
                int_lists: vec![].into(),
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
                callbacks: CompiledCallbacks::interpreted(),
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
            assert_eq!(
                all(
                    &compiled,
                    functions,
                    &execution.execution.program.common.custom_types
                ),
                expected
            );
        }
    }

    #[test]
    fn bool_links_have_their_own_family_and_cannot_name_an_int_function() {
        use crate::plan::execution::function::BoolFunctionId;
        let plan = source_plan(
            "fn choose(value: Int, flag: Bool) { case value < 0 { True -> !flag False -> flag } } pub fn main() { choose(7, True) }",
        );
        let shape = CompiledShape::inspect(plan.bool_function(BoolFunctionId(1)).body()).unwrap();
        let mut compiled = CompiledFunctions {
            function_calls: vec![].into(),
            customs: vec![].into(),
            int_lists: vec![].into(),
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
            callbacks: CompiledCallbacks::interpreted(),
        };

        assert_eq!(
            all(
                &compiled,
                &plan.program.functions,
                &plan.program.common.custom_types
            ),
            Ok(())
        );
        owned_mut(&mut compiled.bools)[0].function = BoolFunctionId(999);
        assert_eq!(
            all(
                &compiled,
                &plan.program.functions,
                &plan.program.common.custom_types
            ),
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

        let mut execution = hosted_plan("pub fn main() { 42 }");
        let program = Arc::get_mut(&mut execution.execution).unwrap();
        let tables = owned_mut(&mut program.program.functions);
        tables.value_returns.int_functions = Vec::from(functions).into();
        assert_eq!(
            all(
                &CompiledFunctions {
                    function_calls: vec![].into(),
                    customs: vec![].into(),
                    int_lists: vec![].into(),
                    ints: Vec::from(compiled).into(),
                    bools: vec![].into(),
                    callbacks: CompiledCallbacks::interpreted(),
                },
                tables,
                &program.program.common.custom_types,
            ),
            Err(CompiledError {
                family: Family::Int,
                function: 0,
                reason: Reason::HostFunction,
            }),
        );
    }

    #[test]
    fn callback_metadata_preserves_constructor_prefixes_and_exact_return_locals() {
        use crate::plan::execution::function::BoolFunctionId;
        use crate::runtime::compiled::tests::metadata_callback;
        let execution = hosted_plan(
            r#"
type Item { Item(Int, Bool) }
fn amount(input: Item) { case input { Item(value, True) -> value + 1 Item(value, False) -> value } }
fn enabled(input: Item) { case input { Item(_, flag) -> flag } }
pub fn main() { #(amount(Item(7, True)), enabled(Item(7, True))) }
"#,
        );
        let program = &execution.execution.program;
        let int_body = graph_body(&program.functions.value_returns.int_functions[0]);
        let bool_body = graph_body(&program.functions.value_returns.bool_functions[0]);
        let integer =
            CompiledShape::inspect_callback(int_body, &program.common.custom_types).unwrap();
        let boolean =
            CompiledShape::inspect_callback(bool_body, &program.common.custom_types).unwrap();
        for change in 0..13 {
            let mut int_points = integer.checkpoints.clone();
            let mut bool_points = boolean.checkpoints.clone();
            if change == 1 {
                int_points[0].customs += 1;
            }
            if change == 2 {
                bool_points[0].bools += 1;
            }
            let mut int_returns = leaf_returns(&int_body.exits);
            let bool_returns = leaf_returns(&bool_body.exits);
            if change == 3 {
                int_returns.pop();
            }
            if change == 4 {
                int_returns[0].0 += 1;
            }
            let mut compiled = CompiledFunctions {
                callbacks: CompiledCallbacks {
                    ints: vec![CompiledCallback {
                        function: IntFunctionId(0),
                        entry: integer.start(integer.graph.entry()),
                        checkpoints: int_points.into(),
                        returns: int_returns.into(),
                        run: metadata_callback,
                    }]
                    .into(),
                    bools: vec![CompiledCallback {
                        function: BoolFunctionId(0),
                        entry: boolean.start(boolean.graph.entry()),
                        checkpoints: bool_points.into(),
                        returns: bool_returns.into(),
                        run: metadata_callback,
                    }]
                    .into(),
                },
                ..CompiledFunctions::interpreted()
            };
            match change {
                5 => owned_mut(&mut compiled.callbacks.ints)[0].entry += 1,
                6 => owned_mut(&mut compiled.callbacks.ints)[0].checkpoints = vec![].into(),
                7 => owned_mut(&mut compiled.callbacks.ints)[0].function = IntFunctionId(999),
                8 => {
                    let row = &compiled.callbacks.ints[0];
                    let duplicate = CompiledCallback {
                        function: row.function,
                        entry: row.entry,
                        checkpoints: row.checkpoints.clone(),
                        returns: row.returns.clone(),
                        run: metadata_callback,
                    };
                    compiled.callbacks.ints = vec![
                        CompiledCallback {
                            function: row.function,
                            entry: row.entry,
                            checkpoints: row.checkpoints.clone(),
                            returns: row.returns.clone(),
                            run: row.run,
                        },
                        duplicate,
                    ]
                    .into();
                }
                9 => owned_mut(&mut compiled.callbacks.bools)[0].entry += 1,
                10 => owned_mut(&mut compiled.callbacks.bools)[0].checkpoints = vec![].into(),
                11 => owned_mut(&mut compiled.callbacks.bools)[0].function = BoolFunctionId(999),
                12 => {
                    let row = &compiled.callbacks.bools[0];
                    let duplicate = CompiledCallback {
                        function: row.function,
                        entry: row.entry,
                        checkpoints: row.checkpoints.clone(),
                        returns: row.returns.clone(),
                        run: metadata_callback,
                    };
                    compiled.callbacks.bools = vec![
                        CompiledCallback {
                            function: row.function,
                            entry: row.entry,
                            checkpoints: row.checkpoints.clone(),
                            returns: row.returns.clone(),
                            run: row.run,
                        },
                        duplicate,
                    ]
                    .into();
                }
                _ => {}
            }
            let expected = match change {
                0 => Ok(()),
                1 => Err(CompiledError {
                    family: Family::IntCallback,
                    function: 0,
                    reason: Reason::Checkpoint(0),
                }),
                2 => Err(CompiledError {
                    family: Family::BoolCallback,
                    function: 0,
                    reason: Reason::Checkpoint(0),
                }),
                3 | 4 => Err(CompiledError {
                    family: Family::IntCallback,
                    function: 0,
                    reason: Reason::Returns,
                }),
                5 | 6 | 8 => Err(CompiledError {
                    family: Family::IntCallback,
                    function: 0,
                    reason: match change {
                        5 => Reason::Entry,
                        6 => Reason::CheckpointCount,
                        _ => Reason::UnorderedTarget,
                    },
                }),
                7 => Err(CompiledError {
                    family: Family::IntCallback,
                    function: 999,
                    reason: Reason::MissingFunction,
                }),
                11 => Err(CompiledError {
                    family: Family::BoolCallback,
                    function: 999,
                    reason: Reason::MissingFunction,
                }),
                _ => Err(CompiledError {
                    family: Family::BoolCallback,
                    function: 0,
                    reason: match change {
                        9 => Reason::Entry,
                        10 => Reason::CheckpointCount,
                        _ => Reason::UnorderedTarget,
                    },
                }),
            };
            if change == 0 {
                let bodies =
                    admit(&compiled, &program.functions, &program.common.custom_types).unwrap();
                assert_eq!((bodies.ints.len(), bodies.bools.len()), (1, 1));
                assert!(std::ptr::eq(bodies.ints[0].body, int_body));
                assert!(std::ptr::eq(bodies.bools[0].body, bool_body));
                assert!(std::ptr::eq(
                    bodies.ints[0].checkpoints,
                    compiled.callbacks.ints[0].checkpoints.as_ref(),
                ));
                assert!(std::ptr::eq(
                    bodies.bools[0].returns,
                    compiled.callbacks.bools[0].returns.as_ref(),
                ));
            }
            assert_eq!(
                all(&compiled, &program.functions, &program.common.custom_types),
                expected
            );
        }
    }

    #[test]
    fn connected_call_metadata_rejects_changed_positions_inputs_outputs_and_prefixes() {
        let plan = source_plan(
            r#"
type Item { Item(Int) }
fn fold(items: List(Item), total: Int, step: fn(Int, Item) -> Int) {
  case items {
    [] -> total
    [head, ..tail] -> fold(tail, step(total, head), step)
  }
}
fn add(total: Int, item: Item) {
  let Item(value) = item
  total + value
}
pub fn main() { fold([Item(2)], 0, add) }
"#,
        );
        assert_eq!(
            crate::runtime::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(2.into())
        );
        let program = &plan.program;
        let body = plan.int_function(IntFunctionId(2)).body();
        let shape = CompiledShape::inspect_custom_loop(body, &program.common.custom_types).unwrap();
        assert_eq!(shape.loop_calls().len(), 1);
        for change in 0..9 {
            let mut calls = shape.loop_calls();
            let mut checkpoints = shape.checkpoints.clone();
            match change {
                1 => calls[0].point += 1,
                2 => {
                    calls[0].args = calls[0].args[..1].to_vec().into();
                }
                3 => owned_mut(&mut calls[0].args).reverse(),
                4 => calls[0].output = ParamLocal::Bool(BoolLocalId(0)),
                5 => calls.clear(),
                6 => calls.push(calls[0].clone()),
                7 => checkpoints[0].custom_lists += 1,
                _ => {}
            }
            let compiled = CompiledFunctions {
                ints: vec![CompiledFunction {
                    function: IntFunctionId(if change == 8 { 0 } else { 2 }),
                    implementation: CompiledImplementation::CustomLoop(
                        Box::new(CustomLoopImplementation {
                            entry: shape.start(shape.graph.entry()),
                            checkpoints: checkpoints.into(),
                            calls: calls.into(),
                            run: metadata_custom_loop,
                        })
                        .into(),
                    ),
                }]
                .into(),
                ..CompiledFunctions::interpreted()
            };
            let expected = match change {
                0 => Ok(()),
                8 => Err(CompiledError {
                    family: Family::Int,
                    function: 0,
                    reason: Reason::UnsupportedGraph,
                }),
                7 => Err(CompiledError {
                    family: Family::Int,
                    function: 2,
                    reason: Reason::Checkpoint(0),
                }),
                _ => Err(CompiledError {
                    family: Family::Int,
                    function: 2,
                    reason: Reason::Calls,
                }),
            };
            assert_eq!(
                all(&compiled, &program.functions, &program.common.custom_types),
                expected
            );
        }
    }
    #[test]
    fn hosted_connected_calls_reject_changed_positions_and_unsupported_targets() {
        let mut execution = hosted_plan(
            r#"
type Item { Item(Int) }
fn fold(items: List(Item), total: Int, step: fn(Int, Item) -> Int) {
  case items {
    [] -> total
    [head, ..tail] -> fold(tail, step(total, head), step)
  }
}
fn add(total: Int, item: Item) {
  let Item(value) = item
  total + value
}
pub fn main() { fold([Item(2)], 0, add) }
"#,
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Int(2.into())
        );
        let program = &execution.execution.program;
        let body = graph_body(&program.functions.value_returns.int_functions[2]);
        let shape = CompiledShape::inspect_custom_loop(body, &program.common.custom_types).unwrap();
        for (function, changed_point, expected) in [
            (2, false, Ok(())),
            (2, true, Err(Reason::Calls)),
            (0, false, Err(Reason::UnsupportedGraph)),
        ] {
            let mut calls = shape.loop_calls();
            if changed_point {
                calls[0].point += 1;
            }
            let compiled = CompiledFunctions {
                ints: vec![CompiledFunction {
                    function: IntFunctionId(function),
                    implementation: CompiledImplementation::CustomLoop(
                        Box::new(CustomLoopImplementation {
                            entry: shape.start(shape.graph.entry()),
                            checkpoints: shape.checkpoints.clone().into(),
                            calls: calls.into(),
                            run: metadata_custom_loop,
                        })
                        .into(),
                    ),
                }]
                .into(),
                ..CompiledFunctions::interpreted()
            };
            assert_eq!(
                all(&compiled, &program.functions, &program.common.custom_types),
                expected.map_err(|reason| CompiledError {
                    family: Family::Int,
                    function,
                    reason
                })
            );
        }
    }

    #[test]
    fn hosted_list_returns_reject_bit_and_custom_loop_kernel_families() {
        let execution = hosted_plan("pub fn main() -> List(Int) { [7, -9] }");
        let program = &execution.execution.program;
        let (function, entry) = &program.functions.list_returns.int_list_functions[0];
        let shape = CompiledShape::inspect(graph_body(entry)).unwrap();
        for implementation in [
            CompiledImplementation::BitArray(BitArrayImplementation {
                entry: shape.start(shape.graph.entry()),
                checkpoints: shape.checkpoints.clone().into(),
                run: metadata_bit_array,
            }),
            CompiledImplementation::CustomLoop(
                Box::new(CustomLoopImplementation {
                    entry: shape.start(shape.graph.entry()),
                    checkpoints: shape.checkpoints.clone().into(),
                    calls: vec![].into(),
                    run: metadata_custom_loop,
                })
                .into(),
            ),
        ] {
            let compiled = CompiledFunctions {
                int_lists: vec![CompiledFunction {
                    function: *function,
                    implementation,
                }]
                .into(),
                ..CompiledFunctions::interpreted()
            };
            assert_eq!(
                all(&compiled, &program.functions, &program.common.custom_types),
                Err(CompiledError {
                    family: Family::IntList,
                    function: function.index,
                    reason: Reason::Kernel,
                })
            );
        }
    }

    #[test]
    fn callback_links_cannot_replace_a_host_or_a_repeating_graph() {
        use crate::runtime::compiled::tests::metadata_callback;
        for source in ["pub fn main() -> Int { main() }", "pub fn main() { 42 }"] {
            let mut execution = hosted_plan(source);
            let program = Arc::get_mut(&mut execution.execution).unwrap();
            let expected = if source.contains("42") {
                owned_mut(&mut program.program.functions)
                    .value_returns
                    .int_functions = vec![ValueFunctionEntry::host(HostedFunctionTarget::Value(
                    HostFunctionId {
                        index: 0,
                        return_: IntLocalId(0),
                        body: PhantomData,
                    },
                ))]
                .into();
                Reason::HostFunction
            } else {
                Reason::UnsupportedGraph
            };
            let compiled = CompiledFunctions {
                callbacks: CompiledCallbacks {
                    ints: vec![CompiledCallback {
                        function: IntFunctionId(0),
                        entry: 0,
                        checkpoints: vec![].into(),
                        returns: vec![].into(),
                        run: metadata_callback,
                    }]
                    .into(),
                    bools: vec![].into(),
                },
                ..CompiledFunctions::interpreted()
            };
            assert_eq!(
                all(
                    &compiled,
                    &program.program.functions,
                    &program.program.common.custom_types
                ),
                Err(CompiledError {
                    family: Family::IntCallback,
                    function: 0,
                    reason: expected
                }),
            );
        }
    }
    #[test]
    #[should_panic(expected = "callback fixture requires return exits")]
    fn integer_leaf_fixture_guard_rejects_a_real_tail_call() {
        let plan = source_plan(
            "fn increment(value: Int) { value + 1 } fn relay(value: Int) { increment(value) } pub fn main() { relay(1) }",
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(2.into())
        );
        leaf_returns(&plan.int_function(IntFunctionId(1)).body().exits);
    }

    #[test]
    #[should_panic(expected = "callback fixture requires return exits")]
    fn boolean_leaf_fixture_guard_rejects_a_real_tail_call() {
        use crate::plan::execution::function::BoolFunctionId;
        let plan = source_plan(
            "fn invert(value: Bool) { !value } fn relay(value: Bool) { invert(value) } pub fn main() { relay(False) }",
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Bool(true)
        );
        leaf_returns(&plan.bool_function(BoolFunctionId(1)).body().exits);
    }

    fn leaf_returns<Local: Clone, Tail>(
        exits: &[crate::plan::execution::function::FunctionExit<Local, Tail>],
    ) -> Vec<Local> {
        exits
            .iter()
            .map(|exit| match exit {
                crate::plan::execution::function::FunctionExit::Return(local) => local.clone(),
                crate::plan::execution::function::FunctionExit::TailCall { .. } => {
                    panic!("callback fixture requires return exits")
                }
            })
            .collect()
    }
}
