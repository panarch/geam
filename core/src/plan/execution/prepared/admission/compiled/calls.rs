use super::{CompiledError, Family, Reason};
use crate::plan::execution::compiled::{CallTarget, CompiledFunctions, CompiledImplementation};
use crate::plan::execution::function::{ExecutionProfile, FunctionTables};
use crate::plan::execution::prepared::codegen::calls::shape::CallProgram;

pub(super) fn all<Profile: ExecutionProfile>(
    compiled: &CompiledFunctions,
    functions: &FunctionTables<Profile>,
) -> Result<(), CompiledError> {
    if compiled.function_calls.is_empty() {
        return Ok(());
    }
    let expected = CallProgram::inspect(functions);
    let mut previous = None;
    for entry in compiled.function_calls.iter() {
        let family = match entry.function {
            CallTarget::Int(_) => Family::Int,
            CallTarget::Bool(_) => Family::Bool,
            CallTarget::IntFunction(_) => Family::IntFunction,
            CallTarget::BoolFunction(_) => Family::BoolFunction,
        };
        let error = |reason| CompiledError {
            family,
            function: entry.function.index(),
            reason,
        };
        if previous.is_some_and(|previous| previous >= entry.function.key()) {
            return Err(error(Reason::UnorderedTarget));
        }
        previous = Some(entry.function.key());
        let CompiledImplementation::FunctionCalls(actual) = &entry.implementation else {
            return Err(error(Reason::Kernel));
        };
        let Some((_, shape)) = expected
            .shapes()
            .find(|(target, _)| *target == entry.function)
        else {
            return Err(error(Reason::UnsupportedGraph));
        };
        if actual.root != shape.root {
            return Err(error(Reason::CallRoot));
        }
        if actual.entry != shape.entry() {
            return Err(error(Reason::Entry));
        }
        if actual.checkpoints.len() != shape.checkpoints.len() {
            return Err(error(Reason::CheckpointCount));
        }
        for (index, (actual, expected)) in actual
            .checkpoints
            .iter()
            .zip(&shape.checkpoints)
            .enumerate()
        {
            if actual != expected {
                return Err(error(Reason::Checkpoint(index)));
            }
        }
        if actual.locals.len() != shape.local_contracts().len()
            || !actual
                .locals
                .iter()
                .zip(shape.local_contracts())
                .all(|(actual, expected)| actual.as_ref() == expected.as_slice())
        {
            return Err(error(Reason::CallLocals));
        }
        if actual.calls.as_ref() != shape.call_contracts().as_slice() {
            return Err(error(Reason::CallMapping));
        }
        if actual.creations.as_ref() != shape.creation_contracts().as_slice() {
            return Err(error(Reason::CallCaptures));
        }
        if actual.returns.as_ref() != shape.return_contracts().as_slice() {
            return Err(error(Reason::CallReturns));
        }
        if actual.tails.as_ref() != shape.tail_contracts().as_slice() {
            return Err(error(Reason::CallTails));
        }
    }
    if let Some((target, _)) = expected
        .shapes()
        .find(|(target, _)| compiled.call(*target).is_none())
    {
        return Err(CompiledError {
            family: match target {
                CallTarget::Int(_) => Family::Int,
                CallTarget::Bool(_) => Family::Bool,
                CallTarget::IntFunction(_) => Family::IntFunction,
                CallTarget::BoolFunction(_) => Family::BoolFunction,
            },
            function: target.index(),
            reason: Reason::MissingFunction,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::admit;
    use super::{CompiledError, Family, Reason, all};
    use crate::plan::execution::Table;
    use crate::plan::execution::compiled::{
        CallTarget, CompiledEntries, CompiledFunction, CompiledFunctions, CompiledImplementation,
        FunctionCallsImplementation, NumericImplementation,
    };
    use crate::plan::execution::function::{
        BoolFunctionFunctionId, BoolFunctionId, IntFunctionFunctionId, IntFunctionId,
    };
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId, ParamLocal};
    use crate::plan::execution::prepared::codegen::calls::shape::CallProgram;
    use crate::runtime::compiled::tests::{metadata_calls, metadata_numeric};
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};

    #[test]
    fn exact_call_contract_admits_and_each_canonical_mapping_field_rejects_corruption() {
        let source = r#"
fn make(offset: Int) { fn(value) { value + offset } }
fn even(value: Int) { case value <= 0 { True -> True False -> odd(value - 1) } }
fn odd(value: Int) { case value <= 0 { True -> False False -> even(value - 1) } }
fn make_predicate(offset: Int) { fn(value) { even(value + offset) } }
pub fn main() { let calculate = make(7) let predicate = make_predicate(4) case predicate(4) { True -> calculate(3) + 1 False -> 0 } }
"#;
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
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut echo).unwrap(),
            crate::Value::Int(11.into())
        );
        assert!(echo.is_empty());
        let program = &execution.execution.program;
        let functions = &program.functions;
        let views = CallProgram::inspect(functions);
        for (fault, reason) in [
            ("unchanged", None),
            ("root", Some(Reason::CallRoot)),
            ("entry", Some(Reason::Entry)),
            ("checkpoint count", Some(Reason::CheckpointCount)),
            ("checkpoint prefix", Some(Reason::Checkpoint(0))),
            ("local mapping", Some(Reason::CallLocals)),
            ("call argument", Some(Reason::CallMapping)),
            ("creation", Some(Reason::CallCaptures)),
            ("return destination", Some(Reason::CallReturns)),
            ("tail mapping", Some(Reason::CallTails)),
            ("implementation", Some(Reason::Kernel)),
            ("foreign target", Some(Reason::UnsupportedGraph)),
            ("duplicate target", Some(Reason::UnorderedTarget)),
        ] {
            let mut rows = views
                .shapes()
                .map(|(target, shape)| {
                    let mut implementation = FunctionCallsImplementation {
                        root: shape.root,
                        entry: shape.entry(),
                        checkpoints: shape.checkpoints.clone().into(),
                        locals: shape
                            .local_contracts()
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<_>>()
                            .into(),
                        calls: shape.call_contracts().into(),
                        creations: shape.creation_contracts().into(),
                        returns: shape.return_contracts().into(),
                        tails: shape.tail_contracts().into(),
                        start: metadata_calls,
                    };
                    if target == CallTarget::Int(IntFunctionId(0)) {
                        match fault {
                            "root" => implementation.root = !implementation.root,
                            "entry" => implementation.entry += 1,
                            "checkpoint count" => implementation.checkpoints = Vec::new().into(),
                            "checkpoint prefix" => {
                                let mut points = shape.checkpoints.clone();
                                points[0].bools += 1;
                                implementation.checkpoints = points.into();
                            }
                            "local mapping" => implementation.locals = Vec::new().into(),
                            "call argument" => {
                                let mut calls = shape.call_contracts();
                                calls[0].args = vec![ParamLocal::Bool(BoolLocalId(0))].into();
                                implementation.calls = calls.into();
                            }
                            "creation" => {
                                let creation = views
                                    .shapes()
                                    .find_map(|(_, shape)| {
                                        shape.creation_contracts().into_iter().next()
                                    })
                                    .unwrap();
                                implementation.creations = vec![creation].into();
                            }
                            "return destination" => {
                                let mut returns = shape.return_contracts();
                                returns[0].value = ParamLocal::Int(IntLocalId(0));
                                implementation.returns = returns.into();
                            }
                            "tail mapping" => {
                                let tail = views
                                    .shapes()
                                    .find_map(|(_, shape)| {
                                        shape.tail_contracts().into_iter().next()
                                    })
                                    .unwrap();
                                implementation.tails = vec![tail].into();
                            }
                            _ => {}
                        }
                    }
                    CompiledFunction {
                        function: target,
                        implementation: CompiledImplementation::FunctionCalls(
                            Box::new(implementation).into(),
                        ),
                    }
                })
                .collect::<Vec<_>>();
            if fault == "implementation" {
                rows[0].implementation = CompiledImplementation::Numeric(NumericImplementation {
                    entry: 0,
                    checkpoints: vec![].into(),
                    run: metadata_numeric,
                });
            }
            if fault == "foreign target" {
                rows[0].function = CallTarget::Int(IntFunctionId(999));
            }
            if fault == "duplicate target" {
                let duplicate = CompiledFunction {
                    function: rows[0].function,
                    implementation: CompiledImplementation::Numeric(NumericImplementation {
                        entry: 0,
                        checkpoints: vec![].into(),
                        run: metadata_numeric,
                    }),
                };
                rows.insert(1, duplicate);
            }
            let compiled = CompiledFunctions {
                native_loops: Table::Static(&[]),
                function_calls: rows.into(),
                ..CompiledFunctions::interpreted()
            };
            if fault == "unchanged" {
                let compiled = Box::leak(Box::new(compiled));
                assert_eq!(all(compiled, functions), Ok(()));
                let entries = CompiledEntries::new(compiled, functions);
                for entry in compiled.function_calls.iter() {
                    let selected = match entry.function {
                        CallTarget::Int(id) => entries.int(id),
                        CallTarget::Bool(id) => entries.bool(id),
                        CallTarget::IntFunction(id) => entries.int_function(id),
                        CallTarget::BoolFunction(id) => entries.bool_function(id),
                    };
                    let (_, shape) = views
                        .shapes()
                        .find(|(target, _)| *target == entry.function)
                        .unwrap();
                    assert_eq!(entry.implementation.checkpoints(), shape.checkpoints);
                    if shape.root {
                        assert!(std::ptr::eq(selected.unwrap(), &entry.implementation));
                    } else {
                        assert!(selected.is_none());
                    }
                }
                assert!(entries.int(IntFunctionId(usize::MAX)).is_none());
                assert!(entries.bool(BoolFunctionId(usize::MAX)).is_none());
                assert!(
                    entries
                        .int_function(IntFunctionFunctionId(usize::MAX))
                        .is_none()
                );
                assert!(
                    entries
                        .bool_function(BoolFunctionFunctionId(usize::MAX))
                        .is_none()
                );
                continue;
            }
            let expected = reason.map_or(Ok(()), |reason| {
                Err(CompiledError {
                    family: Family::Int,
                    function: if fault == "foreign target" { 999 } else { 0 },
                    reason,
                })
            });
            assert_eq!(all(&compiled, functions), expected, "{fault}");
            assert_eq!(
                admit(&compiled, functions, &program.common.custom_types).map(|_| ()),
                expected,
                "{fault}"
            );
        }
        for family in [
            Family::Int,
            Family::Bool,
            Family::IntFunction,
            Family::BoolFunction,
        ] {
            let missing = views
                .shapes()
                .find(|(target, _)| match target {
                    CallTarget::Int(_) => family == Family::Int,
                    CallTarget::Bool(_) => family == Family::Bool,
                    CallTarget::IntFunction(_) => family == Family::IntFunction,
                    CallTarget::BoolFunction(_) => family == Family::BoolFunction,
                })
                .unwrap()
                .0;
            let rows = views
                .shapes()
                .filter(|(target, _)| *target != missing)
                .map(|(target, shape)| CompiledFunction {
                    function: target,
                    implementation: CompiledImplementation::FunctionCalls(
                        Box::new(FunctionCallsImplementation {
                            root: shape.root,
                            entry: shape.entry(),
                            checkpoints: shape.checkpoints.clone().into(),
                            locals: shape
                                .local_contracts()
                                .into_iter()
                                .map(Into::into)
                                .collect::<Vec<_>>()
                                .into(),
                            calls: shape.call_contracts().into(),
                            creations: shape.creation_contracts().into(),
                            returns: shape.return_contracts().into(),
                            tails: shape.tail_contracts().into(),
                            start: metadata_calls,
                        })
                        .into(),
                    ),
                })
                .collect::<Vec<_>>();
            assert!(!rows.is_empty());
            let compiled = CompiledFunctions {
                native_loops: Table::Static(&[]),
                function_calls: rows.into(),
                ..CompiledFunctions::interpreted()
            };
            assert_eq!(
                all(&compiled, functions),
                Err(CompiledError {
                    family,
                    function: missing.index(),
                    reason: Reason::MissingFunction
                })
            );
        }
        assert_eq!(all(&CompiledFunctions::interpreted(), functions), Ok(()));
        let shape = views
            .shapes()
            .find(|(target, _)| *target == CallTarget::Int(IntFunctionId(0)))
            .unwrap()
            .1;
        let misplaced = CompiledFunctions {
            ints: vec![CompiledFunction {
                function: IntFunctionId(0),
                implementation: CompiledImplementation::FunctionCalls(
                    Box::new(FunctionCallsImplementation {
                        root: shape.root,
                        entry: shape.entry(),
                        checkpoints: shape.checkpoints.clone().into(),
                        locals: shape
                            .local_contracts()
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<_>>()
                            .into(),
                        calls: shape.call_contracts().into(),
                        creations: shape.creation_contracts().into(),
                        returns: shape.return_contracts().into(),
                        tails: shape.tail_contracts().into(),
                        start: metadata_calls,
                    })
                    .into(),
                ),
            }]
            .into(),
            ..CompiledFunctions::interpreted()
        };
        assert_eq!(
            admit(&misplaced, functions, &program.common.custom_types).map(|_| ()),
            Err(CompiledError {
                family: Family::Int,
                function: 0,
                reason: Reason::Kernel,
            })
        );
    }
}
