use super::{
    CallContractTarget, CallFamily, CallGroupCodegen, CallInvocation, CallLocal, CallTarget, Code,
    ExecutionGraphProfile, StepKind, call_family, return_families,
};

impl<Graph: ExecutionGraphProfile> CallGroupCodegen<'_, '_, Graph> {
    pub(super) fn write_scalar_native_state(&self, source: &mut Code) {
        if !self.has_native_calls() {
            return;
        }
        source.push_str("#[allow(clippy::large_enum_variant, reason = \"The suspended caller stays in its existing execution allocation.\")]\n");
        source.open("enum FunctionActive {\n");
        source.push_str("Running(FunctionState),\n");
        for family in [CallFamily::Int, CallFamily::Bool] {
            if self.has_native_bridge(family) {
                source.push_str(&format!("{family}Call {{ function: data::function::{family}FunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: {family}Return }},\n"));
                let value = if family == CallFamily::Int {
                    "CallInteger"
                } else {
                    "bool"
                };
                source.push_str(&format!("{family}Return {{ caller: {family}Return, returned: CallNativeReturn<{value}> }},\n"));
            }
        }
        source.close("}\n");
    }

    pub(super) fn has_native_calls(&self) -> bool {
        [CallFamily::Int, CallFamily::Bool]
            .into_iter()
            .any(|family| self.has_native_bridge(family))
    }

    pub(super) fn active(&self, state: &str) -> String {
        if self.has_native_calls() {
            format!("FunctionActive::Running({state})")
        } else {
            state.to_owned()
        }
    }

    pub(super) fn write_native_bridge(
        &self,
        source: &mut Code,
        family: CallFamily,
        native: bool,
        returning: bool,
    ) {
        let (prefix, suffix) = if native { ("Ok(Some(", "))") } else { ("", "") };
        let return_prefix = if returning { "return " } else { "" };
        let return_suffix = if returning { ";" } else { "" };
        if native {
            source.open(&format!("if let CallNativeOps::{family} {{ function: target, native }} = native && *target == function {{\n"));
            source.open("if *budget == 0 {\n");
            source.push_str(&format!("self.active = Some(FunctionActive::{family}Call {{ function, site, input, caller }});\nreturn Ok(Some(CallProgress::Yield(self)));\n"));
            source.close("}\n");
            source.push_str("*budget -= 1;\nlet Some(returned) = native.call(input, site)? else { return Ok(None); };\n");
            source.open("if *budget == 0 {\n");
            source.push_str(&format!("self.active = Some(FunctionActive::{family}Return {{ caller, returned }});\nreturn Ok(Some(CallProgress::Yield(self)));\n"));
            source.close("}\n");
            source.push_str("*budget -= 1;\ncaller.resume(returned.into_value())\n");
            source.alternative("} else {\n");
        }
        source.open(&format!(
            "{return_prefix}{prefix}CallProgress::{family}Scalar {{\n"
        ));
        source.push_str("function, site, input,\n");
        source.open("resume: Box::new(move |value| {\n");
        source.push_str(&format!(
            "self.active = Some({});\nself\n",
            self.active("caller.resume(value)")
        ));
        source.close("}),\n");
        source.close(&format!("}}{suffix}{return_suffix}\n"));
        if native {
            source.close("}\n");
        }
    }

    pub(super) fn has_native_bridge(&self, family: CallFamily) -> bool {
        matches!(family, CallFamily::Int | CallFamily::Bool)
            && self.functions.iter().any(|function| function.shape.calls.iter().any(|call| {
                call_family(call) == family
                    && Self::native_input(call).is_some()
                    && matches!(call.target, CallContractTarget::Static(target) if self.static_callee(target, &call.args).is_none())
            }))
    }

    pub(super) fn native_input(call: &CallInvocation) -> Option<String> {
        if !matches!(
            call.target,
            CallContractTarget::Static(CallTarget::Int(_) | CallTarget::Bool(_))
        ) {
            return None;
        }
        match call.args.as_slice() {
            [CallLocal::Int(id)] => Some(format!("CallNativeInput::Int(int{}.into())", id.0)),
            [CallLocal::Bool(id)] => Some(format!("CallNativeInput::Bool(bool{})", id.0)),
            _ => None,
        }
    }

    pub(super) fn write_advance(&self, source: &mut Code, native: bool) {
        let progress = |expression: &str| {
            if native {
                format!("Ok(Some({expression}))")
            } else {
                expression.to_owned()
            }
        };
        let (prefix, suffix) = if native { ("Ok(Some(", "))") } else { ("", "") };
        if native {
            source.open("fn advance_native(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize, native: &mut CallNativeOps<'_>) -> Result<Option<CallProgress>, CallNativeFailure> {\n");
        } else {
            source.open("fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {\n");
        }
        let tails = self
            .return_families()
            .iter()
            .any(|family| self.has_step(*family, StepKind::Tail));
        // A typed Native request hands off once; only internal state changes
        // require another function_step in this invocation.
        let transitions = (native && self.has_native_calls())
            || self.return_families().into_iter().any(|family| {
                self.has_step(family, StepKind::Call)
                    || self.has_step(family, StepKind::Tail)
                    || self.has_step(family, StepKind::Return)
            });
        let handoff_prefix = if transitions { "return " } else { "" };
        let handoff_suffix = if transitions { ";" } else { "" };
        let mutable = if transitions { "mut " } else { "" };
        let initial_mutable = if self.has_native_calls() { "" } else { mutable };
        source.push_str(&format!(
            "let Some({initial_mutable}active) = self.active.take() else {{ return {}; }};\n",
            progress("CallProgress::Yield(self)")
        ));
        if self.has_native_calls() {
            source.open(&format!("let {mutable}active = match active {{\n"));
            source.push_str("FunctionActive::Running(active) => active,\n");
            for family in [CallFamily::Int, CallFamily::Bool] {
                if self.has_native_bridge(family) {
                    source.open(&format!(
                        "FunctionActive::{family}Call {{ function, site, input, caller }} => {{\n"
                    ));
                    self.write_native_bridge(source, family, native, true);
                    source.close("},\n");
                    source.open(&format!(
                        "FunctionActive::{family}Return {{ caller, returned }} => {{\n"
                    ));
                    source.open("if *budget == 0 {\n");
                    source.push_str(&format!("self.active = Some(FunctionActive::{family}Return {{ caller, returned }});\nreturn {};\n", progress("CallProgress::Yield(self)")));
                    source.close("}\n");
                    source.push_str("*budget -= 1;\ncaller.resume(returned.into_value())\n");
                    source.close("},\n");
                }
            }
            source.close("};\n");
        }
        if transitions {
            source.open("loop {\n");
        }
        if tails {
            source.open("if self.pending_entry {\n");
            source.push_str(&format!("if *budget == 0 {{ self.active = Some({}); return {}; }}\n*budget -= 1;\nself.pending_entry = false;\n", self.active("active"), progress("CallProgress::Yield(self)")));
            source.close("}\n");
        }
        source.open("match function_step(active, ops, budget) {\n");
        for family in self.native_families() {
            source.open(&format!(
                "FunctionStep::{family}Native {{ function, site, arguments, caller }} => {{\n"
            ));
            source.push_str(&format!("let root_tail = caller.is_none() && self.{}.is_empty() && ops.root_tail_entry();\nself.{}_caller = caller;\n", family.return_stack(), family.native_prefix()));
            source.push_str(&format!("{handoff_prefix}{}{handoff_suffix}\n", progress(&format!("CallProgress::{family}Native({family}NativeRequest {{ function, site, arguments, root_tail, execution: self }})"))));
            source.close("},\n");
            source.open(&format!(
                "FunctionStep::{family}NativeComplete {{ value }} => {{\n"
            ));
            for cleared in self.return_families() {
                source.push_str(&format!("self.{}.clear();\n", cleared.return_stack()));
            }
            source.push_str(&format!(
                "{handoff_prefix}{}{handoff_suffix}\n",
                progress(&format!(
                    "CallProgress::Complete {{ output: CallOutput::{family}(value), execution: self }}"
                ))
            ));
            source.close("},\n");
        }
        source.open("FunctionStep::Yield(active) => {\n");
        source.push_str(&format!(
            "self.active = Some({});\n{handoff_prefix}{}{handoff_suffix}\n",
            self.active("active"),
            progress("CallProgress::Yield(self)")
        ));
        source.close("},\n");
        for family in self.return_families() {
            let stack = format!("self.{}", family.return_stack());
            if self.has_step(family, StepKind::Call) {
                source.open(&format!(
                    "FunctionStep::{family}Call {{ callee, caller }} => {{\n"
                ));
                source.push_str(&format!("{stack}.push(caller);\nactive = callee;\n"));
                source.close("},\n");
            }
            if self.has_step(family, StepKind::Tail) {
                source.open(&format!("FunctionStep::{family}Tail {{ callee }} => {{\n"));
                source.push_str(&format!("*budget -= 1;\nself.pending_entry = {stack}.is_empty() && ops.root_tail_entry();\nactive = callee;\n"));
                source.close("},\n");
            }
            if self.has_step(family, StepKind::Return) {
                source.open(&format!("FunctionStep::{family} {{ value }} => {{\n"));
                source.open(&format!("if let Some(caller) = {stack}.pop() {{\n"));
                source.push_str("active = caller.small(value);\n");
                source.alternative("} else {\n");
                for cleared in self.return_families() {
                    source.push_str(&format!("self.{}.clear();\n", cleared.return_stack()));
                }
                let value = if family == CallFamily::Int {
                    "value.into()"
                } else {
                    "value"
                };
                source.push_str(&format!("return {};\n", progress(&format!("CallProgress::Complete {{ output: CallOutput::{family}({value}), execution: self }}"))));
                source.close("}\n");
                source.close("},\n");
            }
            if self.has_step(family, StepKind::Bridge) {
                source.open(&format!("FunctionStep::{family}Bridge {{ function, site, arguments, caller }} => {handoff_prefix}{prefix}CallProgress::{family} {{\n"));
                source.push_str("function, site, arguments,\n");
                source.open("resume: Box::new(move |value| {\n");
                source.push_str(&format!(
                    "self.active = Some({});\nself\n",
                    self.active("caller.resume(value)")
                ));
                source.close("}),\n");
                source.close(&format!("}}{suffix},\n"));
            }
            if self.has_native_bridge(family) {
                source.open(&format!(
                    "FunctionStep::{family}ScalarBridge {{ function, site, input, caller }} => {{\n"
                ));
                if native {
                    source.open("active = {\n");
                }
                self.write_native_bridge(source, family, native, transitions);
                if native {
                    source.close("};\n");
                }
                source.close("},\n");
            }
        }
        if self.has_canonical_step() {
            source.open("FunctionStep::Canonical { target, point, values } => {\n");
            source.open("match target {\n");
            for family in self.return_families() {
                let stack = format!("self.{}", family.return_stack());
                source.open(&format!(
                    "data::compiled::CallTarget::{family}(function) => {{\n"
                ));
                source.open(&format!("if let Some(caller) = {stack}.pop() {{\n"));
                source.push_str("let site = caller.site();\n");
                source.open(&format!(
                    "return {prefix}CallProgress::Interpreted{family} {{\n"
                ));
                source.push_str("function, site, point, values,\n");
                source.open("resume: Box::new(move |value| {\n");
                source.push_str(&format!(
                    "self.active = Some({});\nself\n",
                    self.active("caller.resume(value)")
                ));
                source.close("}),\n");
                source.close(&format!("}}{suffix};\n"));
                source.close("}\n");
                source.push_str(&format!(
                    "{handoff_prefix}{}{handoff_suffix}\n",
                    progress("CallProgress::Interpreted { target, point, values }")
                ));
                source.close("},\n");
            }
            if self.return_families().len() < return_families().len() {
                source.push_str(&format!(
                    "_ => {handoff_prefix}{},\n",
                    progress("CallProgress::Interpreted { target, point, values }")
                ));
            }
            source.close("}\n");
            source.close("},\n");
        }
        source.close("}\n");
        if transitions {
            source.close("}\n");
        }
        source.close("}\n");
    }
}

#[cfg(test)]
mod tests {
    use super::super::shape::CallProgram;
    use super::{CallGroupCodegen, CallTarget, Code};
    use crate::plan::execution::function::{BoolFunctionId, StringFunctionId};
    use crate::{ExecutionPlan, compile_typed_module, plan_module};
    use std::collections::BTreeSet;

    #[test]
    fn a_native_tail_only_body_emits_a_terminal_handoff_without_a_dispatch_loop() {
        use crate::{
            HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
            StatelessHostProfile, StringValue, compile_typed_host_program, plan_host_program,
        };

        let source = r#"
@external(erlang, "example", "append")
fn append(value: String) -> String
fn forward(value: String) -> String { append(value) }
pub fn main() { #(forward("input"), 7) }
"#;
        let providers =
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(StringValue,), StringValue, _>("append", |value: StringValue| value)
            .unwrap()])
            .unwrap();
        let typed = compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            providers,
        )
        .unwrap();
        let execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let program = &execution.execution.program;
        let calls = CallProgram::inspect(
            &program.functions,
            &program.common.custom_types,
            &program.common.value_shapes,
        );
        assert_eq!(calls.functions.len(), 1);
        let forward = &calls.functions[0];
        assert!(forward.shape.calls.is_empty());
        assert!(forward.shape.returns.is_empty());
        assert_eq!(forward.shape.tails.len(), 1);
        assert_eq!(
            forward.shape.tails[0].target,
            CallTarget::String(StringFunctionId(1))
        );
        let native_targets = BTreeSet::from([CallTarget::String(StringFunctionId(1)).key()]);
        let group = CallGroupCodegen::new(vec![forward], vec![forward.target], native_targets);
        let mut generated = Code::default();
        group.write_advance(&mut generated, false);
        assert_eq!(
            generated.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
    match function_step(active, ops, budget) {
        FunctionStep::StringNative { function, site, arguments, caller } => {
            let root_tail = caller.is_none() && self.string_returns.is_empty() && ops.root_tail_entry();
            self.native_caller = caller;
            CallProgress::StringNative(StringNativeRequest { function, site, arguments, root_tail, execution: self })
        },
        FunctionStep::StringNativeComplete { value } => {
            self.string_returns.clear();
            CallProgress::Complete { output: CallOutput::String(value), execution: self }
        },
        FunctionStep::Yield(active) => {
            self.active = Some(active);
            CallProgress::Yield(self)
        },
        FunctionStep::Canonical { target, point, values } => {
            match target {
                data::compiled::CallTarget::String(function) => {
                    if let Some(caller) = self.string_returns.pop() {
                        let site = caller.site();
                        return CallProgress::InterpretedString {
                            function, site, point, values,
                            resume: Box::new(move |value| {
                                self.active = Some(caller.resume(value));
                                self
                            }),
                        };
                    }
                    CallProgress::Interpreted { target, point, values }
                },
                _ => CallProgress::Interpreted { target, point, values },
            }
        },
    }
}
"#
        );
    }

    #[test]
    fn a_canonical_only_body_emits_an_expression_handoff() {
        let source = r#"
fn echo_before_call(calculate: fn(Int) -> Int, value: Int) -> Int {
  let logged = 7
  echo logged
  calculate(value)
}
pub fn main() -> Int {
  let calculate = fn(value) { value + 1 }
  echo_before_call(calculate, 7)
}
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let canonical = program
            .functions
            .iter()
            .find(|function| function.shape.calls.is_empty() && function.shape.returns.is_empty())
            .unwrap();
        let group = CallGroupCodegen::new(vec![canonical], vec![canonical.target], BTreeSet::new());
        let mut generated = Code::default();
        group.write_advance(&mut generated, false);
        assert_eq!(
            generated.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
    match function_step(active, ops, budget) {
        FunctionStep::Yield(active) => {
            self.active = Some(active);
            CallProgress::Yield(self)
        },
        FunctionStep::Canonical { target, point, values } => {
            match target {
                data::compiled::CallTarget::Int(function) => {
                    if let Some(caller) = self.integer_returns.pop() {
                        let site = caller.site();
                        return CallProgress::InterpretedInt {
                            function, site, point, values,
                            resume: Box::new(move |value| {
                                self.active = Some(caller.resume(value));
                                self
                            }),
                        };
                    }
                    CallProgress::Interpreted { target, point, values }
                },
                _ => CallProgress::Interpreted { target, point, values },
            }
        },
    }
}
"#
        );
    }

    #[test]
    fn a_bridge_only_body_emits_an_expression_and_retains_its_caller() {
        let source = r#"
fn invoke(callback: fn(String) -> String, value: String) -> String {
  let output = callback(value)
  echo output
  panic as "caller stopped"
}
pub fn main() -> String {
  invoke(fn(value) { value }, "input")
}
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let invoke = program
            .functions
            .iter()
            .find(|function| function.shape.calls.len() == 1 && function.shape.returns.is_empty())
            .unwrap();
        let group = CallGroupCodegen::new(vec![invoke], vec![invoke.target], BTreeSet::new());
        let mut generated = Code::default();
        group.write_advance(&mut generated, false);
        assert_eq!(
            generated.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
    match function_step(active, ops, budget) {
        FunctionStep::Yield(active) => {
            self.active = Some(active);
            CallProgress::Yield(self)
        },
        FunctionStep::StringBridge { function, site, arguments, caller } => CallProgress::String {
            function, site, arguments,
            resume: Box::new(move |value| {
                self.active = Some(caller.resume(value));
                self
            }),
        },
        FunctionStep::Canonical { target, point, values } => {
            match target {
                data::compiled::CallTarget::String(function) => {
                    if let Some(caller) = self.string_returns.pop() {
                        let site = caller.site();
                        return CallProgress::InterpretedString {
                            function, site, point, values,
                            resume: Box::new(move |value| {
                                self.active = Some(caller.resume(value));
                                self
                            }),
                        };
                    }
                    CallProgress::Interpreted { target, point, values }
                },
                _ => CallProgress::Interpreted { target, point, values },
            }
        },
    }
}
"#
        );
    }
    #[test]
    fn terminal_native_bridge_only_loops_when_resuming_native_calls_locally() {
        use crate::{
            HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
            StatelessHostProfile, compile_typed_host_program, plan_host_program,
        };
        let input = r#"
@external(erlang, "native", "accepted")
fn accepted(flag: Bool) -> Bool
pub fn verify(flag: Bool) -> Bool {
  let assert True = accepted(flag)
  accepted(flag)
}
pub fn main() { let _ = verify(True) Nil }
"#;
        let native = HostProviderModule::<StatelessHostProfile>::new("example", "example")
            .unwrap()
            .with_function::<(bool,), bool, _>("accepted", |value| value)
            .unwrap();
        let typed = compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", input)],
            )],
            HostProviderSet::from_providers([native]).unwrap(),
        )
        .unwrap();
        let mut plan =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let program = &plan.execution().program;
        let calls = CallProgram::inspect(
            &program.functions,
            &program.common.custom_types,
            &program.common.value_shapes,
        );
        let groups = super::super::group::CallGroup::inspect(&calls.functions);
        let group = groups
            .iter()
            .find(|group| {
                group.entries.iter().any(|&entry| {
                    calls.functions[entry].target == CallTarget::Bool(BoolFunctionId(0))
                })
            })
            .unwrap();
        assert_eq!(group.members.len(), 1);
        let codegen = CallGroupCodegen::new(
            group
                .members
                .iter()
                .map(|&member| &calls.functions[member])
                .collect(),
            group
                .entries
                .iter()
                .map(|&entry| calls.functions[entry].target)
                .collect(),
            BTreeSet::new(),
        );
        let mut protocol = Code::default();
        codegen.write_protocol(&mut protocol);
        let expected_protocol = r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Yield(FunctionState),
    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
    BoolScalarBridge { function: data::function::BoolFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: BoolReturn },
}
"#;
        assert_eq!(protocol.as_str(), expected_protocol);
        let mut ordinary = Code::default();
        codegen.write_advance(&mut ordinary, false);
        let expected_ordinary = r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
    let active = match active {
        FunctionActive::Running(active) => active,
        FunctionActive::BoolCall { function, site, input, caller } => {
            return CallProgress::BoolScalar {
                function, site, input,
                resume: Box::new(move |value| {
                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                    self
                }),
            };
        },
        FunctionActive::BoolReturn { caller, returned } => {
            if *budget == 0 {
                self.active = Some(FunctionActive::BoolReturn { caller, returned });
                return CallProgress::Yield(self);
            }
            *budget -= 1;
            caller.resume(returned.into_value())
        },
    };
    match function_step(active, ops, budget) {
        FunctionStep::Yield(active) => {
            self.active = Some(FunctionActive::Running(active));
            CallProgress::Yield(self)
        },
        FunctionStep::BoolScalarBridge { function, site, input, caller } => {
            CallProgress::BoolScalar {
                function, site, input,
                resume: Box::new(move |value| {
                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                    self
                }),
            }
        },
        FunctionStep::Canonical { target, point, values } => {
            match target {
                data::compiled::CallTarget::Bool(function) => {
                    if let Some(caller) = self.boolean_returns.pop() {
                        let site = caller.site();
                        return CallProgress::InterpretedBool {
                            function, site, point, values,
                            resume: Box::new(move |value| {
                                self.active = Some(FunctionActive::Running(caller.resume(value)));
                                self
                            }),
                        };
                    }
                    CallProgress::Interpreted { target, point, values }
                },
                _ => CallProgress::Interpreted { target, point, values },
            }
        },
    }
}
"#;
        assert_eq!(ordinary.as_str(), expected_ordinary);
        let mut native = Code::default();
        codegen.write_advance(&mut native, true);
        let expected_native = r#"fn advance_native(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize, native: &mut CallNativeOps<'_>) -> Result<Option<CallProgress>, CallNativeFailure> {
    let Some(active) = self.active.take() else { return Ok(Some(CallProgress::Yield(self))); };
    let mut active = match active {
        FunctionActive::Running(active) => active,
        FunctionActive::BoolCall { function, site, input, caller } => {
            if let CallNativeOps::Bool { function: target, native } = native && *target == function {
                if *budget == 0 {
                    self.active = Some(FunctionActive::BoolCall { function, site, input, caller });
                    return Ok(Some(CallProgress::Yield(self)));
                }
                *budget -= 1;
                let Some(returned) = native.call(input, site)? else { return Ok(None); };
                if *budget == 0 {
                    self.active = Some(FunctionActive::BoolReturn { caller, returned });
                    return Ok(Some(CallProgress::Yield(self)));
                }
                *budget -= 1;
                caller.resume(returned.into_value())
            } else {
                return Ok(Some(CallProgress::BoolScalar {
                    function, site, input,
                    resume: Box::new(move |value| {
                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                        self
                    }),
                }));
            }
        },
        FunctionActive::BoolReturn { caller, returned } => {
            if *budget == 0 {
                self.active = Some(FunctionActive::BoolReturn { caller, returned });
                return Ok(Some(CallProgress::Yield(self)));
            }
            *budget -= 1;
            caller.resume(returned.into_value())
        },
    };
    loop {
        match function_step(active, ops, budget) {
            FunctionStep::Yield(active) => {
                self.active = Some(FunctionActive::Running(active));
                return Ok(Some(CallProgress::Yield(self)));
            },
            FunctionStep::BoolScalarBridge { function, site, input, caller } => {
                active = {
                    if let CallNativeOps::Bool { function: target, native } = native && *target == function {
                        if *budget == 0 {
                            self.active = Some(FunctionActive::BoolCall { function, site, input, caller });
                            return Ok(Some(CallProgress::Yield(self)));
                        }
                        *budget -= 1;
                        let Some(returned) = native.call(input, site)? else { return Ok(None); };
                        if *budget == 0 {
                            self.active = Some(FunctionActive::BoolReturn { caller, returned });
                            return Ok(Some(CallProgress::Yield(self)));
                        }
                        *budget -= 1;
                        caller.resume(returned.into_value())
                    } else {
                        return Ok(Some(CallProgress::BoolScalar {
                            function, site, input,
                            resume: Box::new(move |value| {
                                self.active = Some(FunctionActive::Running(caller.resume(value)));
                                self
                            }),
                        }));
                    }
                };
            },
            FunctionStep::Canonical { target, point, values } => {
                match target {
                    data::compiled::CallTarget::Bool(function) => {
                        if let Some(caller) = self.boolean_returns.pop() {
                            let site = caller.site();
                            return Ok(Some(CallProgress::InterpretedBool {
                                function, site, point, values,
                                resume: Box::new(move |value| {
                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                                    self
                                }),
                            }));
                        }
                        return Ok(Some(CallProgress::Interpreted { target, point, values }));
                    },
                    _ => return Ok(Some(CallProgress::Interpreted { target, point, values })),
                }
            },
        }
    }
}
"#;
        assert_eq!(native.as_str(), expected_native);

        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut plan, &mut (), &mut echo).unwrap(),
            crate::Value::Nil
        );
        assert!(echo.is_empty());
    }
}
