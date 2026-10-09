use super::super::KernelKind;
use super::local::capture_expression;
use super::shape::{
    CallBoolean, CallFunction, CallLocal, CallPoint, CallScalar, CallTerminator, CallTest,
    CallableTarget,
};
use super::{
    CallGroupCodegen, Code, ExecutionGraphProfile, IntListInstruction, IntListTest, Rust,
    call_family, canonical, function_name, function_state, local_name, pattern, state, state_name,
    write_scalar,
};
use std::collections::BTreeSet;

impl<Graph: ExecutionGraphProfile> CallGroupCodegen<'_, '_, Graph> {
    pub(super) fn write_function(&self, source: &mut Code, function: &CallFunction<'_, Graph>) {
        for (point, locals) in function
            .shape
            .locals
            .iter()
            .enumerate()
            .filter(|(point, _)| self.global_point(function, *point))
        {
            source.push_str(&format!(
                "FunctionState::{}{} => {}_run({}State::Point{point}{}, ops, budget),\n",
                state_name(function.target, point),
                pattern(locals),
                function_name(function.target),
                function_state(function.target),
                pattern(locals)
            ));
        }
    }

    pub(super) fn write_body(&self, source: &mut Code, function: &CallFunction<'_, Graph>) {
        let owner = function_state(function.target);
        source.open(&format!("enum {owner}State {{\n"));
        for (point, locals) in function
            .shape
            .locals
            .iter()
            .enumerate()
            .filter(|(point, _)| self.local_point(function, *point))
        {
            source.push_str(&format!("Point{point}{},\n", super::fields(locals)));
        }
        source.close("}\n");
        let starts = segment_starts(function);
        let repeats = function.kernel.is_none()
            && function
                .shape
                .points
                .iter()
                .enumerate()
                .any(|(point, action)| {
                    matches!(action, CallPoint::Terminator(_))
                        || (!starts.contains(&point) && action.is_inline())
                });
        let mutable = if repeats { "mut " } else { "" };
        let ops = if self.uses_ops(function) {
            "ops"
        } else {
            "_ops"
        };
        source.open(&format!("fn {}_run({mutable}active: {owner}State, {ops}: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {{\n", function_name(function.target)));
        if repeats {
            source.open("loop {\n");
        }
        source.open("match active {\n");
        for (entry, locals) in function
            .shape
            .locals
            .iter()
            .enumerate()
            .filter(|(point, _)| self.local_point(function, *point))
        {
            source.open(&format!(
                "{owner}State::Point{entry}{} => {{\n",
                pattern(locals)
            ));
            let mut point = entry;
            loop {
                let action = &function.shape.points[point];
                self.write_point(source, function, point, repeats);
                if function.kernel.is_some() || !action.is_inline() {
                    break;
                }
                let next = point + 1;
                if !starts.contains(&entry) {
                    source.push_str(&format!(
                        "active = {owner}State::Point{next}{};\ncontinue;\n",
                        pattern(&function.shape.locals[next])
                    ));
                    break;
                }
                point = next;
            }
            source.close("},\n");
        }
        source.close("}\n");
        if repeats {
            source.close("}\n");
        }
        source.close("}\n");
    }

    // Range kernels retain their owned scratch across yields. Only their entry
    // needs ordinary locals unless external entry can select another checkpoint.
    // Numeric kernels still publish their yielded locals in shared states.
    pub(super) fn global_point(&self, function: &CallFunction<'_, Graph>, point: usize) -> bool {
        if function
            .kernel
            .as_ref()
            .is_some_and(|kernel| kernel.kind != KernelKind::Numeric)
        {
            return point == function.shape.entry()
                || self.entry_targets.contains(&function.target);
        }
        function.kernel.is_some()
            || !matches!(function.shape.points[point], CallPoint::Interpreted)
            || self.entry_targets.contains(&function.target)
            || segment_starts(function).contains(&point)
    }

    pub(super) fn local_point(&self, function: &CallFunction<'_, Graph>, point: usize) -> bool {
        self.global_point(function, point)
            || point.checked_sub(1).is_some_and(|previous| {
                function.shape.points[previous].is_inline()
                    && !segment_starts(function).contains(&previous)
            })
    }

    fn uses_ops(&self, function: &CallFunction<'_, Graph>) -> bool {
        function.shape.calls.iter().any(|call| {
            !matches!(call.target, super::CallContractTarget::Static(_))
                && self
                    .entries
                    .iter()
                    .any(|entry| entry.matches(call_family(call), &call.args))
        }) || function.shape.calls.iter().any(|call| matches!(call.target, super::CallContractTarget::Static(target) if self.is_native(target, &call.args)))
            || function.shape.tails.iter().any(|tail| self.is_native(tail.target, &tail.args))
            || function.kernel.is_some()
            || !function.shape.creations.is_empty()
            || function.shape.points.iter().any(|point| {
                matches!(
                    point,
                    CallPoint::Scalar(CallScalar::IntList(_))
                        | CallPoint::Scalar(CallScalar::List { .. })
                        | CallPoint::Scalar(CallScalar::Index { .. })
                        | CallPoint::Scalar(CallScalar::Boolean(
                            _,
                            CallBoolean::Test(CallTest::Equal {
                                left: CallLocal::BoolList { .. }
                                    | CallLocal::FloatList { .. }
                                    | CallLocal::StringList { .. }
                                    | CallLocal::BitArrayList { .. }
                                    | CallLocal::UtfCodepointList { .. }
                                    | CallLocal::NilList { .. },
                                ..
                            })
                        ))
                        | CallPoint::Terminator(CallTerminator::Test {
                            test: CallTest::Equal {
                                left: CallLocal::BoolList { .. }
                                    | CallLocal::FloatList { .. }
                                    | CallLocal::StringList { .. }
                                    | CallLocal::BitArrayList { .. }
                                    | CallLocal::UtfCodepointList { .. }
                                    | CallLocal::NilList { .. },
                                ..
                            },
                            ..
                        })
                        | CallPoint::Scalar(CallScalar::Boolean(
                            _,
                            CallBoolean::Test(CallTest::IntList(IntListTest::Equal { .. }))
                        ))
                        | CallPoint::Terminator(CallTerminator::Test {
                            test: CallTest::IntList(IntListTest::Equal { .. }),
                            ..
                        })
                )
            })
    }

    fn write_point(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        point: usize,
        repeats: bool,
    ) {
        let prefix = if repeats { "return " } else { "" };
        let suffix = if repeats { ";" } else { "" };
        if let Some(numeric) = &function.kernel {
            source.open(&format!("{prefix}{{\n"));
            self.write_kernel_step(source, function, numeric, point);
            source.close(&format!("}}{suffix}\n"));
            return;
        }
        let action = &function.shape.points[point];
        let locals = &function.shape.locals[point];
        let checkpoint = function.shape.checkpoints[point];
        if !matches!(action, CallPoint::Interpreted) {
            source.push_str(&format!(
                "if *budget == 0 {{ return FunctionStep::Yield({}); }}\n",
                state(function.target, point, locals)
            ));
            if let CallPoint::Scalar(CallScalar::IntList(IntListInstruction::Index {
                output,
                list,
                index,
            })) = action
            {
                source.open(&format!(
                    "let int{} = match ops.lists().index(&int_list{}, {index}) {{\n",
                    output.0, list.0
                ));
                source.push_str("Some(value) => value,\n");
                source.push_str(&format!(
                    "None => return {},\n",
                    canonical(function.target, checkpoint, locals)
                ));
                source.close("};\n");
            }
            if let CallPoint::Scalar(CallScalar::Index {
                output,
                list,
                index,
            }) = action
            {
                source.open(&format!(
                    "let {} = match ops.primitive_lists().{}_index(&{}, {index}) {{\n",
                    if matches!(output, super::CallLocal::Nil(_)) {
                        "()".to_owned()
                    } else {
                        local_name(output)
                    },
                    super::list_family(list).0,
                    local_name(&list.canonical())
                ));
                source.push_str("Some(value) => value,\n");
                source.push_str(&format!(
                    "None => return {},\n",
                    canonical(function.target, checkpoint, locals)
                ));
                source.close("};\n");
            }
            if !matches!(action, CallPoint::Tail(_)) {
                source.push_str("*budget -= 1;\n");
            }
        }
        match action {
            CallPoint::Scalar(instruction) => {
                write_scalar(source, instruction);
                let outputs: Vec<_> = match instruction {
                    CallScalar::Integer(output, _) => vec![format!("int{}", output.0)],
                    CallScalar::Region { outputs, .. } => outputs
                        .iter()
                        .map(|output| format!("int{}", output.0))
                        .collect(),
                    _ => Vec::new(),
                };
                if !outputs.is_empty() {
                    let checks = outputs
                        .iter()
                        .map(|local| {
                            format!(
                                "{local} < i128::from(i64::MIN) || {local} > i128::from(i64::MAX)"
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" || ");
                    source.push_str(&format!(
                        "if {checks} {{ return {}; }}\n",
                        canonical(
                            function.target,
                            function.shape.checkpoints[point + 1],
                            &function.shape.locals[point + 1]
                        )
                    ));
                }
            }
            CallPoint::Create(index) => {
                let creation = &function.shape.creations[*index];
                let (family, target) = match creation.target {
                    CallableTarget::Int(id) => ("int", Rust::expression(&id)),
                    CallableTarget::Bool(id) => ("bool", Rust::expression(&id)),
                    CallableTarget::Float(id) => ("float", Rust::expression(&id)),
                    CallableTarget::String(id) => ("string", Rust::expression(&id)),
                    CallableTarget::BitArray(id) => ("bit_array", Rust::expression(&id)),
                    CallableTarget::UtfCodepoint(id) => ("utf_codepoint", Rust::expression(&id)),
                    CallableTarget::Nil(id) => ("nil", Rust::expression(&id)),
                };
                let method = if creation.reference {
                    "reference"
                } else {
                    "closure"
                };
                let captures = if creation.reference {
                    String::new()
                } else {
                    format!(
                        ", vec![{}]",
                        creation
                            .captures
                            .iter()
                            .map(capture_expression)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                source.push_str(&format!(
                    "let {} = ops.{family}_{method}({target}, {}{captures});\n",
                    local_name(&creation.output),
                    Rust::expression(&creation.type_)
                ));
            }
            CallPoint::Terminator(terminator) => {
                source.open("active = {\n");
                self.write_terminator(source, function, terminator);
                source.close("};\n");
                source.push_str("continue;\n");
            }
            CallPoint::Call(index) => {
                source.open(&format!("{prefix}{{\n"));
                self.write_call(source, function, &function.shape.calls[*index]);
                source.close(&format!("}}{suffix}\n"));
            }
            CallPoint::Tail(index) => {
                source.open(&format!("{prefix}{{\n"));
                self.write_tail(source, function, *index);
                source.close(&format!("}}{suffix}\n"));
            }
            CallPoint::Return(index) => {
                source.open(&format!("{prefix}{{\n"));
                self.write_return(source, function, *index);
                source.close(&format!("}}{suffix}\n"));
            }
            CallPoint::Interpreted => source.push_str(&format!(
                "{prefix}{}{suffix}\n",
                canonical(function.target, checkpoint, locals)
            )),
        }
    }
}

fn segment_starts<Graph: ExecutionGraphProfile>(
    function: &CallFunction<'_, Graph>,
) -> BTreeSet<usize> {
    std::iter::once(function.shape.entry())
        .chain(function.shape.starts.values().copied())
        .chain(function.shape.calls.iter().map(|call| call.point + 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{CallGroupCodegen, CallPoint};
    use crate::plan::execution::prepared::codegen::calls::shape::CallProgram;
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
    use std::collections::BTreeSet;

    #[test]
    fn callee_only_string_kernel_yields_in_owned_state_and_preserves_external_checkpoints() {
        let source = r#"
fn classify(value: String) -> Int {
  case value {
    "tag:" <> rest -> case rest { "x" -> 1 _ -> 2 }
    _ -> 0
  }
}
pub fn main() { let result = classify("tag:x") result + 1 }
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
        let program = &execution.execution.program;
        let calls = CallProgram::inspect(
            &program.functions,
            &program.common.custom_types,
            &program.common.value_shapes,
        );
        let callee = calls
            .functions
            .iter()
            .find(|function| {
                function.kernel.is_some()
                    && function
                        .shape
                        .points
                        .iter()
                        .any(|point| matches!(point, CallPoint::Interpreted))
            })
            .expect("the source requires a String kernel with checkpointed slice operations");
        let group = CallGroupCodegen::new(vec![callee], Vec::new(), BTreeSet::new());
        assert_eq!(
            (0..callee.shape.points.len())
                .filter(|&point| group.global_point(callee, point))
                .collect::<Vec<_>>(),
            vec![callee.shape.entry()],
        );
        let external = CallGroupCodegen::new(vec![callee], vec![callee.target], BTreeSet::new());
        assert_eq!(
            (0..callee.shape.points.len())
                .filter(|&point| external.global_point(callee, point))
                .collect::<Vec<_>>(),
            (0..callee.shape.points.len()).collect::<Vec<_>>(),
        );
    }
}
