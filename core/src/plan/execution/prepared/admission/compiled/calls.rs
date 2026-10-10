use super::{CompiledError, Family, Reason};
use crate::plan::execution::compiled::{CallTarget, CompiledFunctions, CompiledImplementation};
use crate::plan::execution::function::{ExecutionProfile, FunctionTables};
use crate::plan::execution::prepared::codegen::calls::shape::CallProgram;
use crate::plan::execution::type_::{CustomTypeTable, ValueShapeTable};

pub(super) fn all<Profile: ExecutionProfile>(
    compiled: &CompiledFunctions,
    functions: &FunctionTables<Profile>,
    custom_types: &CustomTypeTable,
    value_shapes: &ValueShapeTable,
) -> Result<(), CompiledError> {
    if compiled.function_calls.is_empty() {
        return Ok(());
    }
    let expected = CallProgram::inspect(functions, custom_types, value_shapes);
    let mut previous = None;
    for entry in compiled.function_calls.iter() {
        let family = match entry.function {
            CallTarget::Custom(_) => Family::Custom,
            CallTarget::Tuple(_) => Family::Tuple,
            CallTarget::Int(_) => Family::Int,
            CallTarget::Bool(_) => Family::Bool,
            CallTarget::IntFunction(_) => Family::IntFunction,
            CallTarget::BoolFunction(_) => Family::BoolFunction,
            CallTarget::Float(_) => Family::Float,
            CallTarget::FloatFunction(_) => Family::FloatFunction,
            CallTarget::String(_) => Family::String,
            CallTarget::StringFunction(_) => Family::StringFunction,
            CallTarget::BitArray(_) => Family::BitArray,
            CallTarget::BitArrayFunction(_) => Family::BitArrayFunction,
            CallTarget::UtfCodepoint(_) => Family::UtfCodepoint,
            CallTarget::UtfCodepointFunction(_) => Family::UtfCodepointFunction,
            CallTarget::Nil(_) => Family::Nil,
            CallTarget::NilFunction(_) => Family::NilFunction,
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
                CallTarget::Custom(_) => Family::Custom,
                CallTarget::Tuple(_) => Family::Tuple,
                CallTarget::Int(_) => Family::Int,
                CallTarget::Bool(_) => Family::Bool,
                CallTarget::IntFunction(_) => Family::IntFunction,
                CallTarget::BoolFunction(_) => Family::BoolFunction,
                CallTarget::Float(_) => Family::Float,
                CallTarget::FloatFunction(_) => Family::FloatFunction,
                CallTarget::String(_) => Family::String,
                CallTarget::StringFunction(_) => Family::StringFunction,
                CallTarget::BitArray(_) => Family::BitArray,
                CallTarget::BitArrayFunction(_) => Family::BitArrayFunction,
                CallTarget::UtfCodepoint(_) => Family::UtfCodepoint,
                CallTarget::UtfCodepointFunction(_) => Family::UtfCodepointFunction,
                CallTarget::Nil(_) => Family::Nil,
                CallTarget::NilFunction(_) => Family::NilFunction,
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
    use crate::plan::execution::graph::{
        BoolLocalId, FunctionCapture, IntListLocalId, IntLocalId, ListLocal, ParamLocal,
    };
    use crate::plan::execution::prepared::codegen::calls::shape::CallProgram;
    use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
    use crate::runtime::compiled::tests::{metadata_calls, metadata_numeric};
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};

    #[test]
    fn native_and_nullary_call_contracts_reject_changed_targets_shapes_sites_and_destinations() {
        use crate::plan::HostCallSite;
        use crate::plan::execution::compiled::CallContractTarget;
        use crate::plan::execution::function::StringFunctionId;
        use crate::plan::execution::graph::StringLocalId;
        use crate::plan::execution::type_::CustomValueShapeId;
        use crate::{HostProviderModule, StringValue};

        #[derive(Clone, Copy, Debug)]
        enum NativeFault {
            Target,
            Arity,
            Order,
            Site,
            Destination,
        }
        #[derive(Clone, Copy, Debug)]
        enum Fault {
            Unchanged,
            NullaryShape,
            Native(NativeFault),
            ReturnFamily,
            TailArguments,
            TailSite,
        }

        let source = r#"
pub type Direction { Before After }
@external(erlang, "example", "append")
fn append(value: String, suffix: String) -> String
fn number(value: Int) -> Int { value }
fn forward(value: String, direction: Direction) -> String { append(value, "!") }
pub fn main() -> String {
  let _ = number(7)
  let first = forward("input", Before)
  let second = forward(first, After)
  let result = append(second, "?")
  result
}
"#;
        let hosts =
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(StringValue, StringValue), StringValue, _>(
                "append",
                |value: StringValue, suffix: StringValue| {
                    format!("{}{}", value.as_str().unwrap(), suffix.as_str().unwrap()).into()
                },
            )
            .unwrap()])
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            hosts,
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        let host = crate::execution_fixture::TestHost::default();
        assert_eq!(
            host.block_on(crate::runtime::run_hosted_main(
                &mut execution,
                &host,
                &mut (),
                &mut echo
            ))
            .unwrap()
            .try_into_value()
            .unwrap(),
            crate::Value::String("input!!?".into())
        );
        assert!(echo.is_empty());
        let program = &execution.execution.program;
        let views = CallProgram::inspect(
            &program.functions,
            &program.common.custom_types,
            &program.common.value_shapes,
        );
        for (fault, reason) in [
            (Fault::Unchanged, None),
            (Fault::NullaryShape, Some(Reason::CallLocals)),
            (
                Fault::Native(NativeFault::Target),
                Some(Reason::CallMapping),
            ),
            (Fault::Native(NativeFault::Arity), Some(Reason::CallMapping)),
            (Fault::Native(NativeFault::Order), Some(Reason::CallMapping)),
            (Fault::Native(NativeFault::Site), Some(Reason::CallMapping)),
            (
                Fault::Native(NativeFault::Destination),
                Some(Reason::CallMapping),
            ),
            (Fault::ReturnFamily, Some(Reason::CallReturns)),
            (Fault::TailArguments, Some(Reason::CallTails)),
            (Fault::TailSite, Some(Reason::CallTails)),
        ] {
            let mut failed_target = None;
            let rows = views
                .shapes()
                .map(|(target, shape)| {
                    let mut body = FunctionCallsImplementation {
                        root: shape.root,
                        entry: shape.entry(),
                        checkpoints: shape.checkpoints.clone().into(),
                        locals: shape
                            .local_contracts()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        calls: shape.call_contracts().into(),
                        creations: shape.creation_contracts().into(),
                        returns: shape.return_contracts().into(),
                        tails: shape.tail_contracts().into(),
                        start: metadata_calls,
                    };
                    if failed_target.is_none() {
                        match fault {
                            Fault::NullaryShape => {
                                let mut locals = shape.local_contracts();
                                if let Some(local) =
                                    locals.iter_mut().flatten().find_map(|local| match local {
                                        ParamLocal::Custom(local) => Some(local),
                                        _ => None,
                                    })
                                {
                                    local.shape.shape_id = CustomValueShapeId(999);
                                    body.locals = locals.into_iter().map(Into::into).collect();
                                    failed_target = Some(target);
                                }
                            }
                            Fault::Native(native_fault) => {
                                let mut calls = shape.call_contracts();
                                if let Some(call) = calls.iter_mut().find(|call| {
                                    matches!(
                                        call.args.as_ref(),
                                        [ParamLocal::String(_), ParamLocal::String(_)]
                                    )
                                }) {
                                    match native_fault {
                                        NativeFault::Target => {
                                            call.target = CallContractTarget::Static(
                                                CallTarget::String(StringFunctionId(999)),
                                            )
                                        }
                                        NativeFault::Arity => {
                                            call.args = vec![call.args[0].clone()].into()
                                        }
                                        NativeFault::Order => {
                                            call.args =
                                                vec![call.args[1].clone(), call.args[0].clone()]
                                                    .into()
                                        }
                                        NativeFault::Site => {
                                            call.site = HostCallSite::from_static(
                                                "example",
                                                "changed",
                                                crate::SourceSpan::new(0, 1),
                                            )
                                        }
                                        NativeFault::Destination => {
                                            call.output = ParamLocal::String(StringLocalId(999))
                                        }
                                    }
                                    body.calls = calls.into();
                                    failed_target = Some(target);
                                }
                            }
                            Fault::ReturnFamily => {
                                let mut returns = shape.return_contracts();
                                returns
                                    .first_mut()
                                    .expect("the selected source body returns a value")
                                    .value = ParamLocal::Bool(BoolLocalId(0));
                                body.returns = returns.into();
                                failed_target = Some(target);
                            }
                            Fault::TailArguments | Fault::TailSite if !body.tails.is_empty() => {
                                let mut tails = shape.tail_contracts();
                                if matches!(fault, Fault::TailArguments) {
                                    tails[0].args = Vec::new().into();
                                } else {
                                    tails[0].site = HostCallSite::from_static(
                                        "example",
                                        "changed",
                                        crate::SourceSpan::new(0, 1),
                                    );
                                }
                                body.tails = tails.into();
                                failed_target = Some(target);
                            }
                            _ => {}
                        }
                    }
                    CompiledFunction {
                        function: target,
                        implementation: CompiledImplementation::FunctionCalls(
                            Box::new(body).into(),
                        ),
                    }
                })
                .collect::<Vec<_>>();
            let expected = reason.map_or(Ok(()), |reason| {
                Err(CompiledError {
                    family: if matches!(failed_target, Some(CallTarget::Int(_))) {
                        Family::Int
                    } else {
                        Family::String
                    },
                    function: failed_target
                        .expect("every listed fault has a source contract")
                        .index(),
                    reason,
                })
            });
            assert_eq!(
                all(
                    &CompiledFunctions {
                        function_calls: rows.into(),
                        ..CompiledFunctions::interpreted()
                    },
                    &program.functions,
                    &program.common.custom_types,
                    &program.common.value_shapes,
                ),
                expected,
                "{fault:?}",
            );
        }
    }

    #[test]
    fn list_call_admission_rejects_wrong_typed_locals_captures_arguments_and_prefixes() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let values = [3]
  let bias = 2
  let calculate = fn(input) {
    let value = identity(input)
    case values { [first, ..] -> value + first + bias [] -> value + bias }
  }
  calculate(5)
}
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
        let execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let functions = &execution.execution.program.functions;
        let views = CallProgram::inspect(
            functions,
            &execution.execution.program.common.custom_types,
            &execution.execution.program.common.value_shapes,
        );
        for (fault, reason) in [
            ("unchanged", None),
            ("list type", Some(Reason::CallLocals)),
            ("list local", Some(Reason::CallLocals)),
            ("list prefix", Some(Reason::Checkpoint(0))),
            ("capture source", Some(Reason::CallCaptures)),
            ("call argument", Some(Reason::CallMapping)),
        ] {
            let mut rows = views
                .shapes()
                .map(|(target, shape)| {
                    (
                        target,
                        FunctionCallsImplementation {
                            root: shape.root,
                            entry: shape.entry(),
                            checkpoints: shape.checkpoints.clone().into(),
                            locals: shape
                                .local_contracts()
                                .into_iter()
                                .map(Into::into)
                                .collect(),
                            calls: shape.call_contracts().into(),
                            creations: shape.creation_contracts().into(),
                            returns: shape.return_contracts().into(),
                            tails: shape.tail_contracts().into(),
                            start: metadata_calls,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let failed_target = match fault {
                "list type" | "list local" => {
                    let (target, body) = rows
                        .iter_mut()
                        .find(|(_, body)| {
                            body.locals.iter().flatten().any(|local| {
                                matches!(local, ParamLocal::List(ListLocal::Int { .. }))
                            })
                        })
                        .expect("the source retains a typed integer List local");
                    let mut locals = body
                        .locals
                        .iter()
                        .map(|locals| locals.to_vec())
                        .collect::<Vec<_>>();
                    let (local, type_id) = locals
                        .iter_mut()
                        .flatten()
                        .find_map(|local| match local {
                            ParamLocal::List(ListLocal::Int { local, type_id }) => {
                                Some((local, type_id))
                            }
                            _ => None,
                        })
                        .unwrap();
                    if fault == "list type" {
                        type_id.list_type.0 += 1;
                    } else {
                        local.0 += 1;
                    }
                    body.locals = locals.into_iter().map(Into::into).collect();
                    Some(*target)
                }
                "list prefix" => {
                    let (target, body) = &mut rows[0];
                    let mut checkpoints = body.checkpoints.to_vec();
                    checkpoints[0].int_lists += 1;
                    body.checkpoints = checkpoints.into();
                    Some(*target)
                }
                "capture source" => {
                    let (target, body) = rows
                        .iter_mut()
                        .find(|(_, body)| !body.creations.is_empty())
                        .expect("the source creates a callable capturing the List and bias");
                    let mut creations = body.creations.to_vec();
                    let creation = &mut creations[0];
                    let mut captures = creation.captures.to_vec();
                    let mut changed = 0;
                    for capture in &mut captures {
                        if let FunctionCapture::IntList { source, .. } = capture {
                            source.0 += 1;
                            changed += 1;
                        }
                    }
                    assert_eq!(changed, 1, "only the List capture source changes");
                    creation.captures = captures.into();
                    body.creations = creations.into();
                    Some(*target)
                }
                "call argument" => {
                    let (target, body) = rows
                        .iter_mut()
                        .find(|(_, body)| !body.calls.is_empty())
                        .expect("the source calls an integer function");
                    let mut calls = body.calls.to_vec();
                    calls[0].args = vec![ParamLocal::List(ListLocal::Int {
                        local: IntListLocalId(0),
                        type_id: IntListTypeId {
                            list_type: ListTypeId(0),
                        },
                    })]
                    .into();
                    body.calls = calls.into();
                    Some(*target)
                }
                _ => None,
            };
            let rows = rows
                .into_iter()
                .map(|(function, body)| CompiledFunction {
                    function,
                    implementation: CompiledImplementation::FunctionCalls(Box::new(body).into()),
                })
                .collect::<Vec<_>>();
            let compiled = CompiledFunctions {
                function_calls: rows.into(),
                ..CompiledFunctions::interpreted()
            };
            let expected = reason
                .map(|reason| {
                    let target =
                        failed_target.expect("the corruption must reach its intended contract");
                    CompiledError {
                        family: Family::Int,
                        function: target.index(),
                        reason,
                    }
                })
                .map_or(Ok(()), Err);
            assert_eq!(
                all(
                    &compiled,
                    functions,
                    &execution.execution.program.common.custom_types,
                    &execution.execution.program.common.value_shapes
                ),
                expected,
                "{fault}"
            );
        }
    }

    #[test]
    fn exact_call_contract_admits_and_each_canonical_mapping_field_rejects_corruption() {
        let source = r#"
pub type Box { Box(Int) }
fn keep_box(value: Box) -> Box { value }
fn keep_pair(value: #(Int, Bool)) -> #(Int, Bool) { value }
fn make(offset: Int) { fn(value) { value + offset } }
fn even(value: Int) { case value <= 0 { True -> True False -> odd(value - 1) } }
fn odd(value: Int) { case value <= 0 { True -> False False -> even(value - 1) } }
fn make_predicate(offset: Int) { fn(value) { even(value + offset) } }
fn keep_float(value: Float) -> Float { value }
fn make_float(value: Float) { fn() { keep_float(value) } }
fn keep_string(value: String) -> String { value }
fn make_string(value: String) { fn() { keep_string(value) } }
fn keep_bits(value: BitArray) -> BitArray { value }
fn make_bits(value: BitArray) { fn() { keep_bits(value) } }
fn keep_codepoint(value: UtfCodepoint) -> UtfCodepoint { value }
fn make_codepoint(value: UtfCodepoint) { fn() { keep_codepoint(value) } }
fn keep_nil(value: Nil) -> Nil { value }
fn make_nil(value: Nil) { fn() { keep_nil(value) } }
fn codepoint() { let assert <<value:utf8_codepoint>> = <<"λ":utf8>> value }
pub fn main() { let _ = keep_box(Box(7)) let _ = keep_pair(#(7, True)) let _ = make_float(1.5)() let _ = make_string("label")() let _ = make_bits(<<1>>)() let _ = make_codepoint(codepoint())() let _ = make_nil(Nil)() let calculate = make(7) let predicate = make_predicate(4) case predicate(4) { True -> calculate(3) + 1 False -> 0 } }
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
        let views = CallProgram::inspect(
            functions,
            &execution.execution.program.common.custom_types,
            &execution.execution.program.common.value_shapes,
        );
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
                assert_eq!(
                    all(
                        compiled,
                        functions,
                        &program.common.custom_types,
                        &program.common.value_shapes
                    ),
                    Ok(())
                );
                let entries = CompiledEntries::new(compiled, functions);
                for entry in compiled.function_calls.iter() {
                    let selected = match entry.function {
                        CallTarget::Custom(id) => compiled.call_root(CallTarget::Custom(id)),
                        CallTarget::Tuple(id) => compiled.call_root(CallTarget::Tuple(id)),
                        CallTarget::Int(id) => entries.int(id),
                        CallTarget::Bool(id) => entries.bool(id),
                        CallTarget::IntFunction(id) => entries.int_function(id),
                        CallTarget::BoolFunction(id) => entries.bool_function(id),
                        CallTarget::Float(id) => entries.float(id),
                        CallTarget::FloatFunction(id) => entries.float_function(id),
                        CallTarget::String(id) => entries.string(id),
                        CallTarget::StringFunction(id) => entries.string_function(id),
                        CallTarget::BitArray(id) => entries.bit_array(id),
                        CallTarget::BitArrayFunction(id) => entries.bit_array_function(id),
                        CallTarget::UtfCodepoint(id) => entries.utf_codepoint(id),
                        CallTarget::UtfCodepointFunction(id) => entries.utf_codepoint_function(id),
                        CallTarget::Nil(id) => entries.nil(id),
                        CallTarget::NilFunction(id) => entries.nil_function(id),
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
            assert_eq!(
                all(
                    &compiled,
                    functions,
                    &execution.execution.program.common.custom_types,
                    &execution.execution.program.common.value_shapes
                ),
                expected,
                "{fault}"
            );
            assert_eq!(
                admit(
                    &compiled,
                    functions,
                    &program.common.custom_types,
                    &program.common.value_shapes
                )
                .map(|_| ()),
                expected,
                "{fault}"
            );
        }
        for family in [
            Family::Custom,
            Family::Tuple,
            Family::Int,
            Family::Bool,
            Family::IntFunction,
            Family::BoolFunction,
            Family::Float,
            Family::String,
            Family::BitArray,
            Family::UtfCodepoint,
            Family::Nil,
            Family::FloatFunction,
            Family::StringFunction,
            Family::BitArrayFunction,
            Family::UtfCodepointFunction,
            Family::NilFunction,
        ] {
            let missing = views
                .shapes()
                .find(|(target, _)| match target {
                    CallTarget::Custom(_) => family == Family::Custom,
                    CallTarget::Tuple(_) => family == Family::Tuple,
                    CallTarget::Int(_) => family == Family::Int,
                    CallTarget::Bool(_) => family == Family::Bool,
                    CallTarget::IntFunction(_) => family == Family::IntFunction,
                    CallTarget::BoolFunction(_) => family == Family::BoolFunction,
                    CallTarget::Float(_) => family == Family::Float,
                    CallTarget::FloatFunction(_) => family == Family::FloatFunction,
                    CallTarget::String(_) => family == Family::String,
                    CallTarget::StringFunction(_) => family == Family::StringFunction,
                    CallTarget::BitArray(_) => family == Family::BitArray,
                    CallTarget::BitArrayFunction(_) => family == Family::BitArrayFunction,
                    CallTarget::UtfCodepoint(_) => family == Family::UtfCodepoint,
                    CallTarget::UtfCodepointFunction(_) => family == Family::UtfCodepointFunction,
                    CallTarget::Nil(_) => family == Family::Nil,
                    CallTarget::NilFunction(_) => family == Family::NilFunction,
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
                all(
                    &compiled,
                    functions,
                    &execution.execution.program.common.custom_types,
                    &execution.execution.program.common.value_shapes
                ),
                Err(CompiledError {
                    family,
                    function: missing.index(),
                    reason: Reason::MissingFunction
                })
            );
        }
        assert_eq!(
            all(
                &CompiledFunctions::interpreted(),
                functions,
                &program.common.custom_types,
                &program.common.value_shapes
            ),
            Ok(())
        );
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
            admit(
                &misplaced,
                functions,
                &program.common.custom_types,
                &program.common.value_shapes
            )
            .map(|_| ()),
            Err(CompiledError {
                family: Family::Int,
                function: 0,
                reason: Reason::Kernel,
            })
        );
    }
}
