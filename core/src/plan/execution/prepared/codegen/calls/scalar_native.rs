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
        prefix: &str,
        suffix: &str,
    ) {
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
        source.open(&format!("return {prefix}CallProgress::{family}Scalar {{\n"));
        source.push_str("function, site, input,\n");
        source.open("resume: Box::new(move |value| {\n");
        source.push_str(&format!(
            "self.active = Some({});\nself\n",
            self.active("caller.resume(value)")
        ));
        source.close("}),\n");
        source.close(&format!("}}{suffix};\n"));
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
        // A String Native request hands off once; only internal state changes
        // require another function_step in this invocation.
        let transitions = self.has_native_calls()
            || self.return_families().into_iter().any(|family| {
                self.has_step(family, StepKind::Call)
                    || self.has_step(family, StepKind::Tail)
                    || self.has_step(family, StepKind::Return)
            });
        let handoff_prefix = if transitions { "return " } else { "" };
        let handoff_suffix = if transitions { ";" } else { "" };
        let mutable = if !self.has_native_calls() && transitions {
            "mut "
        } else {
            ""
        };
        if self.has_native() {
            self.write_native_delivery(source, native);
        }
        source.push_str(&format!(
            "let Some({mutable}active) = self.active.take() else {{ return {}; }};\n",
            progress("CallProgress::Yield(self)")
        ));
        if self.has_native_calls() {
            source.open("let mut active = match active {\n");
            source.push_str("FunctionActive::Running(active) => active,\n");
            for family in [CallFamily::Int, CallFamily::Bool] {
                if self.has_native_bridge(family) {
                    source.open(&format!(
                        "FunctionActive::{family}Call {{ function, site, input, caller }} => {{\n"
                    ));
                    self.write_native_bridge(source, family, native, prefix, suffix);
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
        if self.has_native() {
            source.open("FunctionStep::StringNative { function, site, arguments, caller } => {\n");
            source.push_str("let root_tail = caller.is_none() && self.string_returns.is_empty() && ops.root_tail_entry();\nself.native_caller = caller;\n");
            source.push_str(&format!("{handoff_prefix}{}{handoff_suffix}\n", progress("CallProgress::StringNative(StringNativeRequest { function, site, arguments, root_tail, execution: self })")));
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
                self.write_native_bridge(source, family, native, prefix, suffix);
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
    use crate::plan::execution::function::StringFunctionId;
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
        let native_strings = BTreeSet::from([1]);
        let group = CallGroupCodegen::new(vec![forward], vec![forward.target], native_strings);
        let mut generated = Code::default();
        group.write_advance(&mut generated, false);
        assert_eq!(
            generated.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    if let Some(result) = self.native_result.take() {
        if let Some(caller) = self.native_caller.take().or_else(|| self.string_returns.pop()) {
            self.active = Some(caller.small(result));
        } else {
            self.string_returns.clear();
            return CallProgress::Complete { output: CallOutput::String(result), execution: self };
        }
    }
    let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
    match function_step(active, ops, budget) {
        FunctionStep::StringNative { function, site, arguments, caller } => {
            let root_tail = caller.is_none() && self.string_returns.is_empty() && ops.root_tail_entry();
            self.native_caller = caller;
            CallProgress::StringNative(StringNativeRequest { function, site, arguments, root_tail, execution: self })
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
}
