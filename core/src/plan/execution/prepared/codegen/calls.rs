pub(in crate::plan::execution::prepared) mod shape;

use self::shape::{
    CallBoolean, CallFunction, CallInvocation, CallLocal, CallPoint, CallProgram, CallScalar,
    CallTerminator, CallTest, CallableTarget, Capture,
};
use super::int_list::{IntListInstruction, IntListTest};
use super::shape::{NumericInteger, NumericOperation};
use super::{Code, CompiledShape, length_expression, tuple};
use crate::plan::execution::compiled::{CallContractTarget, CallTarget, CompiledCheckpoint};
use crate::plan::execution::function::{ExecutionGraphProfile, ExecutionProfile, FunctionTables};
use crate::plan::execution::graph::{
    ArithmeticNode, ArithmeticOperand, Edge, IntegerOperand, ParamLocal,
};
use crate::plan::execution::prepared::rust::Rust;
use crate::plan::execution::storage::Table;
use std::fmt;

pub(in crate::plan::execution::prepared) struct CallCodegen<'graph, Graph: ExecutionGraphProfile> {
    functions: Vec<CallFunction<'graph, Graph>>,
    entries: Vec<CallableEntry>,
}

struct CallableEntry {
    family: CallFamily,
    parameters: Vec<CallLocal>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CallFamily {
    Int,
    Bool,
    IntFunction,
    BoolFunction,
}

impl CallFamily {
    fn name(self) -> &'static str {
        match self {
            Self::Int => "Int",
            Self::Bool => "Bool",
            Self::IntFunction => "IntFunction",
            Self::BoolFunction => "BoolFunction",
        }
    }
    fn return_stack(self) -> &'static str {
        match self {
            Self::Int => "integer_returns",
            Self::Bool => "boolean_returns",
            Self::IntFunction => "integer_function_returns",
            Self::BoolFunction => "boolean_function_returns",
        }
    }
    fn value_type(self) -> &'static str {
        match self {
            Self::Int => "i128",
            Self::Bool => "bool",
            Self::IntFunction => "IntCallable",
            Self::BoolFunction => "BoolCallable",
        }
    }
}

impl fmt::Display for CallFamily {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str(self.name())
    }
}

#[derive(Clone, Copy)]
enum StepKind {
    Call,
    Tail,
    Return,
    Bridge,
}

impl<'graph, Graph: ExecutionGraphProfile> CallCodegen<'graph, Graph> {
    pub(super) fn new<Profile: ExecutionProfile<Graph = Graph>>(
        functions: &'graph FunctionTables<Profile>,
    ) -> Self {
        let mut codegen = Self {
            functions: CallProgram::inspect(functions).functions,
            entries: Vec::new(),
        };
        for function in &codegen.functions {
            for call in &function.shape.calls {
                if matches!(call.target, CallContractTarget::Static(_)) {
                    continue;
                }
                let family = call_family(call);
                if codegen
                    .functions
                    .iter()
                    .any(|callee| callee.accepts_call(call))
                    && !codegen
                        .entries
                        .iter()
                        .any(|entry| entry.matches(family, &call.args))
                {
                    codegen.entries.push(CallableEntry {
                        family,
                        parameters: call.args.clone(),
                    });
                }
            }
        }
        codegen
    }

    pub(super) fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    pub(super) fn is_root(&self, target: CallTarget) -> bool {
        self.functions
            .iter()
            .any(|function| function.target == target && function.shape.root)
    }

    pub(super) fn write_code(&self, source: &mut Code) {
        if self.is_empty() {
            return;
        }
        let mut imports = vec![
            "CallExecution",
            "CallInputs",
            "CallOps",
            "CallProgress",
            "CallStorage",
        ];
        let canonical = self.has_canonical_step();
        for (family, name) in [
            (CallFamily::IntFunction, "IntCallable"),
            (CallFamily::BoolFunction, "BoolCallable"),
        ] {
            if canonical
                || self.functions.iter().any(|function| {
                    function.shape.locals.iter().flatten().any(|local| {
                        matches!(
                            (family, local),
                            (CallFamily::IntFunction, CallLocal::IntFunction { .. })
                                | (CallFamily::BoolFunction, CallLocal::BoolFunction { .. })
                        )
                    })
                })
            {
                imports.push(name);
            }
        }
        if canonical {
            imports.push("CallInteger");
        }
        if canonical
            || return_families().iter().any(|family| {
                self.has_step(*family, StepKind::Tail) || self.has_step(*family, StepKind::Bridge)
            })
        {
            imports.push("CallValues");
        }
        if return_families()
            .iter()
            .any(|family| self.has_step(*family, StepKind::Return))
        {
            imports.push("CallOutput");
        }
        if return_families()
            .iter()
            .any(|family| self.has_step(*family, StepKind::Bridge))
        {
            imports.push("CallArguments");
        }
        if !self.entries.is_empty() {
            imports.push("CallCaptureInputs");
        }
        if self.functions.iter().any(|function| {
            function
                .shape
                .creations
                .iter()
                .any(|creation| !creation.captures.is_empty())
        }) {
            imports.push("CallCapture");
        }
        imports.sort_unstable();
        source.push_str(&format!(
            "use data::compiled::calls::{{{}}};\n",
            imports.join(", ")
        ));
        if self
            .functions
            .iter()
            .any(|function| function.shape.has_int_lists())
        {
            source.push_str("use data::compiled::int_list::IntList;\n");
        }
        let canonical_return = self.functions.iter().any(|function| {
            function
                .shape
                .calls
                .iter()
                .any(|call| matches!(call.output, CallLocal::Int(_)))
        });
        source.open("enum FunctionState {\n");
        for function in &self.functions {
            for (point, locals) in function.shape.locals.iter().enumerate() {
                source.push_str(&format!(
                    "{}{},\n",
                    state_name(function.target, point),
                    fields(locals)
                ));
            }
        }
        if canonical_return {
            source.push_str("Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },\n");
        }
        source.close("}\n");
        if return_families()
            .iter()
            .any(|family| self.has_step(*family, StepKind::Tail))
        {
            source.open("impl FunctionState {\n");
            source.open("fn values(self) -> CallValues {\n");
            source.open("match self {\n");
            for function in &self.functions {
                for (point, locals) in function.shape.locals.iter().enumerate() {
                    source.open(&format!(
                        "Self::{}{} => {{\n",
                        state_name(function.target, point),
                        pattern(locals)
                    ));
                    source.push_str(&format!("{}\n", values(locals, false)));
                    source.close("},\n");
                }
            }
            if canonical_return {
                source.push_str("Self::Canonical { values, .. } => values,\n");
            }
            source.close("}\n");
            source.close("}\n");
            source.close("}\n");
        }
        for family in return_families() {
            self.write_continuations(source, family);
        }
        self.write_protocol(source);
        self.write_execution(source);
        let ops = if !self.entries.is_empty()
            || self.functions.iter().any(|function| {
                function.numeric.is_some()
                    || !function.shape.creations.is_empty()
                    || function.shape.points.iter().any(|point| {
                        matches!(
                            point,
                            CallPoint::Scalar(CallScalar::IntList(_))
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
            }) {
            "ops"
        } else {
            "_ops"
        };
        source.open(&format!("fn function_step(active: FunctionState, {ops}: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {{\n"));
        source.open("match active {\n");
        if canonical_return {
            source.push_str("FunctionState::Canonical { target, point, values } => FunctionStep::Canonical { target, point, values },\n");
        }
        for function in &self.functions {
            self.write_function(source, function);
        }
        source.close("}\n");
        source.close("}\n");
        self.write_entries(source);
        for function in &self.functions {
            if let Some(numeric) = &function.numeric {
                self.write_numeric(source, function, numeric);
            }
            self.write_start(source, function);
        }
    }

    fn write_execution(&self, source: &mut Code) {
        source.open("struct FunctionExecution {\n");
        source.push_str("active: Option<FunctionState>,\n");
        for family in return_families() {
            source.push_str(&format!(
                "{}: Vec<{family}Return>,\n",
                family.return_stack()
            ));
        }
        source.close("}\n");
        source.open("impl FunctionExecution {\n");
        source.open("fn new(active: FunctionState) -> Self {\n");
        source.open("Self {\n");
        source.push_str("active: Some(active),\n");
        for family in return_families() {
            source.push_str(&format!("{}: Vec::new(),\n", family.return_stack()));
        }
        source.close("}\n");
        source.close("}\n");
        source.close("}\n");
        source.open("impl CallExecution for FunctionExecution {\n");
        source.open("fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {\n");
        source.push_str("if self.active.is_some() { return false; }\n");
        source.open("let active = match target {\n");
        for function in &self.functions {
            source.push_str(&format!(
                "{} => {}_state(point, values),\n",
                Rust::expression(&function.target),
                function_name(function.target)
            ));
        }
        source.push_str("_ => None,\n");
        source.close("};\n");
        source.push_str("let Some(active) = active else { return false; };\nself.active = Some(active);\ntrue\n");
        source.close("}\n");
        source.open("fn retained_bytes(&self) -> usize {\n");
        let capacities = return_families()
            .map(|family| {
                format!(
                    "self.{}.capacity() * std::mem::size_of::<{family}Return>()",
                    family.return_stack()
                )
            })
            .join(" + ");
        source.push_str(&format!("std::mem::size_of::<Self>() + {capacities}\n"));
        source.close("}\n");
        self.write_advance(source);
        source.close("}\n");
    }

    fn write_advance(&self, source: &mut Code) {
        let has_next = self.has_next_step();
        let loops = has_next
            || return_families().into_iter().any(|family| {
                [StepKind::Call, StepKind::Tail, StepKind::Return]
                    .into_iter()
                    .any(|kind| self.has_step(family, kind))
            });
        let mutable = if loops { "mut " } else { "" };
        let return_prefix = if loops { "return " } else { "" };
        let return_suffix = if loops { ";" } else { "" };
        source.open("fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {\n");
        source.push_str(&format!("let Some({mutable}active) = self.active.take() else {{ return CallProgress::Yield(self); }};\n"));
        if loops {
            source.open("loop {\n");
        }
        source.open("match function_step(active, ops, budget) {\n");
        if has_next {
            source.push_str("FunctionStep::Next(next) => active = next,\n");
        }
        source.open("FunctionStep::Yield(active) => {\n");
        source.push_str(&format!(
            "self.active = Some(active);\n{return_prefix}CallProgress::Yield(self){return_suffix}\n"
        ));
        source.close("},\n");
        for family in return_families() {
            let stack = format!("self.{}", family.return_stack());
            if self.has_step(family, StepKind::Call) {
                source.open(&format!(
                    "FunctionStep::{family}Call {{ callee, caller }} => {{\n"
                ));
                source.push_str(&format!("{stack}.push(caller);\nactive = callee;\n"));
                source.close("},\n");
            }
            if self.has_step(family, StepKind::Tail) {
                source.open(&format!(
                    "FunctionStep::{family}Tail {{ callee, completed, point }} => {{\n"
                ));
                source.open(&format!("if {stack}.is_empty() {{\n"));
                source.push_str(
                    "return CallProgress::Interpreted { point, values: completed.values() };\n",
                );
                source.close("}\n");
                source.push_str("*budget -= 1;\nactive = callee;\n");
                source.close("},\n");
            }
            if self.has_step(family, StepKind::Return) {
                source.open(&format!("FunctionStep::{family} {{ value, exit }} => {{\n"));
                source.open(&format!("if let Some(caller) = {stack}.pop() {{\n"));
                source.push_str("active = caller.small(value);\n");
                source.alternative("} else {\n");
                for cleared in return_families() {
                    source.push_str(&format!("self.{}.clear();\n", cleared.return_stack()));
                }
                let value = if family == CallFamily::Int {
                    "value.into()"
                } else {
                    "value"
                };
                source.push_str(&format!("return CallProgress::Complete {{ exit, output: CallOutput::{family}({value}), execution: self }};\n"));
                source.close("}\n");
                source.close("},\n");
            }
            if self.has_step(family, StepKind::Bridge) {
                source.open(&format!("FunctionStep::{family}Bridge {{ function, site, arguments, caller }} => {return_prefix}CallProgress::{family} {{\n"));
                source.push_str("function, site, arguments,\n");
                source.open("resume: Box::new(move |value| {\n");
                source.push_str("self.active = Some(caller.resume(value));\nself\n");
                source.close("}),\n");
                source.close("},\n");
            }
        }
        if self.has_canonical_step() {
            source.open("FunctionStep::Canonical { target, point, values } => {\n");
            source.open("match target {\n");
            for family in return_families() {
                let stack = format!("self.{}", family.return_stack());
                source.open(&format!(
                    "data::compiled::CallTarget::{family}(function) => {{\n"
                ));
                source.open(&format!("if let Some(caller) = {stack}.pop() {{\n"));
                source.push_str("let site = caller.site();\n");
                source.open(&format!("return CallProgress::Interpreted{family} {{\n"));
                source.push_str("function, site, point, values,\n");
                source.open("resume: Box::new(move |value| {\n");
                source.push_str("self.active = Some(caller.resume(value));\nself\n");
                source.close("}),\n");
                source.close("};\n");
                source.close("}\n");
                source.push_str(&format!(
                    "{return_prefix}CallProgress::Interpreted {{ point, values }}{return_suffix}\n"
                ));
                source.close("},\n");
            }
            source.close("}\n");
            source.close("},\n");
        }
        source.close("}\n");
        if loops {
            source.close("}\n");
        }
        source.close("}\n");
    }

    fn write_protocol(&self, source: &mut Code) {
        source.push_str("#[allow(clippy::large_enum_variant, reason = \"Typed locals stay inline to avoid allocating at each generated step.\")]\n");
        source.open("enum FunctionStep {\n");
        if self.has_next_step() {
            source.push_str("Next(FunctionState),\n");
        }
        source.push_str("Yield(FunctionState),\n");
        if self.has_canonical_step() {
            source.push_str("Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },\n");
        }
        for family in return_families() {
            if self.has_step(family, StepKind::Call) {
                source.push_str(&format!(
                    "{family}Call {{ callee: FunctionState, caller: {family}Return }},\n"
                ));
            }
            if self.has_step(family, StepKind::Tail) {
                source.push_str(&format!("{family}Tail {{ callee: FunctionState, completed: FunctionState, point: data::compiled::CompiledCheckpoint }},\n"));
            }
            if self.has_step(family, StepKind::Return) {
                source.push_str(&format!(
                    "{family} {{ value: {}, exit: data::graph::BlockGraphExitId }},\n",
                    family.value_type()
                ));
            }
            if self.has_step(family, StepKind::Bridge) {
                source.push_str(&format!("{family}Bridge {{ function: data::function::{family}FunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: {family}Return }},\n"));
            }
        }
        source.close("}\n");
    }

    fn has_next_step(&self) -> bool {
        self.functions.iter().any(|function| {
            function.numeric.is_none()
                && function
                    .shape
                    .points
                    .iter()
                    .enumerate()
                    .any(|(point, action)| match action {
                        CallPoint::Scalar(_) => Self::following_return(function, point).is_none(),
                        CallPoint::Create(_) | CallPoint::Terminator(_) => true,
                        CallPoint::Call(_)
                        | CallPoint::Tail(_)
                        | CallPoint::Return(_)
                        | CallPoint::Interpreted => false,
                    })
        })
    }

    fn following_return(function: &CallFunction<'_, Graph>, point: usize) -> Option<usize> {
        match function.shape.points[point + 1] {
            CallPoint::Return(index)
                if function.shape.checkpoints[point].block
                    == function.shape.checkpoints[point + 1].block =>
            {
                Some(index)
            }
            _ => None,
        }
    }

    fn has_canonical_step(&self) -> bool {
        self.functions.iter().any(|function| {
            function.numeric.is_some()
                || function
                    .shape
                    .calls
                    .iter()
                    .any(|call| matches!(call.output, CallLocal::Int(_)))
                || function.shape.points.iter().any(|action| match action {
                    CallPoint::Scalar(CallScalar::Integer(..))
                    | CallPoint::Scalar(CallScalar::IntList(IntListInstruction::Index {
                        ..
                    }))
                    | CallPoint::Interpreted => true,
                    CallPoint::Scalar(CallScalar::Region { outputs, .. }) => !outputs.is_empty(),
                    CallPoint::Tail(index) => {
                        let tail = &function.shape.tails[*index];
                        self.static_callee(tail.target, &tail.args).is_none()
                    }
                    _ => false,
                })
        })
    }

    fn has_step(&self, family: CallFamily, kind: StepKind) -> bool {
        self.functions.iter().any(|function| match kind {
            StepKind::Return => {
                target_family(function.target) == family && !function.shape.returns.is_empty()
            }
            StepKind::Tail => {
                target_family(function.target) == family
                    && function
                        .shape
                        .tails
                        .iter()
                        .any(|tail| self.static_callee(tail.target, &tail.args).is_some())
            }
            StepKind::Call | StepKind::Bridge => function.shape.calls.iter().any(|call| {
                if call_family(call) != family {
                    return false;
                }
                let direct = match call.target {
                    CallContractTarget::Static(target) => {
                        self.static_callee(target, &call.args).is_some()
                    }
                    _ => self
                        .functions
                        .iter()
                        .any(|callee| callee.accepts_call(call)),
                };
                match kind {
                    StepKind::Call => direct,
                    _ => !matches!(call.target, CallContractTarget::Static(_)) || !direct,
                }
            }),
        })
    }

    fn static_callee(
        &self,
        target: CallTarget,
        args: &[CallLocal],
    ) -> Option<&CallFunction<'graph, Graph>> {
        self.functions.iter().find(|callee| {
            let parameters = &callee.shape.locals[callee.shape.entry()];
            callee.target == target
                && callee.shape.parameter_count == parameters.len()
                && callee.matches_parameters(args)
        })
    }

    fn write_numeric_step(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        numeric: &CompiledShape<'_, Graph>,
        point: usize,
    ) {
        let name = format!(
            "numeric_{}_{}",
            target_family(function.target).name().to_lowercase(),
            function.target.index()
        );
        let checkpoint = function.shape.checkpoints[point];
        let progress = if point == function.shape.entry() {
            let inputs = tuple(
                (0..checkpoint.ints)
                    .map(|local| format!("int{local}"))
                    .chain((0..checkpoint.bools).map(|local| format!("bool{local}"))),
            );
            format!("{name}_entry({inputs}, values, budget)")
        } else {
            let ints = (0..checkpoint.ints)
                .map(|local| format!("int{local}"))
                .collect::<Vec<_>>()
                .join(", ");
            let bools = (0..checkpoint.bools)
                .map(|local| format!("bool{local}"))
                .collect::<Vec<_>>()
                .join(", ");
            source.push_str(&format!("let values = ops.numeric();\nvalues.ints.clear();\nvalues.ints.extend_from_slice(&[{ints}]);\nvalues.bools.clear();\nvalues.bools.extend_from_slice(&[{bools}]);\n"));
            format!(
                "{name}({}, values, budget)",
                numeric.start(checkpoint.block) + checkpoint.instruction
            )
        };
        if point == function.shape.entry() {
            source.push_str("let values = ops.numeric();\n");
        }
        source.push_str(&format!(
            "let progress = {progress};\n{}_numeric(progress, values)\n",
            function_name(function.target)
        ));
    }

    fn write_numeric(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        numeric: &CompiledShape<'_, Graph>,
    ) {
        source.open(&format!("fn {}_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {{\n", function_name(function.target)));
        source.open(&format!("const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; {}] = [\n", numeric.checkpoints.len()));
        for checkpoint in &numeric.checkpoints {
            let point = function.shape.starts[&checkpoint.block.index()] + checkpoint.instruction;
            let assignments = (0..checkpoint.ints)
                .map(|local| format!("int{local}: values.ints[{local}]"))
                .chain(
                    (0..checkpoint.bools)
                        .map(|local| format!("bool{local}: values.bools[{local}]")),
                )
                .collect::<Vec<_>>()
                .join(", ");
            let values = if checkpoint.ints == 0 && checkpoint.bools == 0 {
                "_values"
            } else {
                "values"
            };
            source.push_str(&format!(
                "|{values}| FunctionState::{} {{ {assignments} }},\n",
                state_name(function.target, point)
            ));
        }
        source.close("];\n");
        source.open(&format!("const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; {}] = [\n", function.numeric_returns.len()));
        for local in &function.numeric_returns {
            let column = if matches!(local, CallLocal::Int(_)) {
                "ints"
            } else {
                "bools"
            };
            let local = local_id(local);
            source.push_str(&format!(
                "|exit, values| FunctionStep::{} {{ value: values.{column}[{local}.0], exit }},\n",
                target_family(function.target)
            ));
        }
        source.close("];\n");
        source.open("match progress {\n");
        source.push_str("data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),\n");
        source.open("data::compiled::CompiledProgress::Interpreted(point) => {\n");
        source.open(&format!(
            "const POINTS: [data::compiled::CompiledCheckpoint; {}] = [\n",
            numeric.checkpoints.len()
        ));
        for checkpoint in &numeric.checkpoints {
            source.push_str(&format!("{},\n", Rust::expression(checkpoint)));
        }
        source.close("];\n");
        source.push_str(&format!("FunctionStep::Canonical {{ target: {}, point: POINTS[point], values: CallValues {{ ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() }} }}\n", Rust::expression(&function.target)));
        source.close("},\n");
        source.push_str(
            "data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),\n",
        );
        source.close("}\n");
        source.close("}\n");
    }

    fn write_start(&self, source: &mut Code, function: &CallFunction<'_, Graph>) {
        let inputs = if function
            .shape
            .locals
            .iter()
            .any(|locals| !locals.is_empty())
        {
            "values"
        } else {
            "_values"
        };
        source.open(&format!(
            "fn {}_state(point: usize, {inputs}: CallInputs<'_>) -> Option<FunctionState> {{\n",
            function_name(function.target)
        ));
        source.open("let active = match point {\n");
        for (point, locals) in function.shape.locals.iter().enumerate() {
            let entry_call = match &function.shape.points[point] {
                CallPoint::Call(index) if function.shape.root => {
                    let call = &function.shape.calls[*index];
                    match call.target {
                        CallContractTarget::IntValue(local) => {
                            Some((call, "int_function_target", local.0))
                        }
                        CallContractTarget::BoolValue(local) => {
                            Some((call, "bool_function_target", local.0))
                        }
                        CallContractTarget::Static(_) => None,
                    }
                }
                _ => None,
            };
            let guard = entry_call.map(|(call, getter, local)| {
                let targets = self
                    .functions
                    .iter()
                    .filter(|callee| callee.accepts_call(call))
                    .map(|callee| target_id(callee.target))
                    .collect::<Vec<_>>()
                    .join(" | ");
                (getter, local, targets)
            });
            let assignments = locals
                .iter()
                .map(|local| format!("{}: {}", local_name(local), load_value(local)))
                .collect::<Vec<_>>()
                .join(", ");
            if let Some((getter, local, targets)) = guard {
                source.open(&format!("{point} => {{\n"));
                if targets.is_empty() {
                    source.push_str("return None\n");
                } else {
                    source.push_str(&format!(
                        "if !matches!(values.{getter}({local})?, {targets}) {{ return None; }}\n"
                    ));
                    source.push_str(&format!(
                        "FunctionState::{} {{ {assignments} }}\n",
                        state_name(function.target, point)
                    ));
                }
                source.close("},\n");
            } else {
                source.push_str(&format!(
                    "{point} => FunctionState::{} {{ {assignments} }},\n",
                    state_name(function.target, point)
                ));
            }
        }
        source.push_str("_ => return None,\n");
        source.close("};\n");
        source.push_str("Some(active)\n");
        source.close("}\n");
        source.open(&format!("fn {}_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {{\n", function_name(function.target)));
        source.push_str(&format!("if let Some(execution) = storage.reuse({}, point, values) {{ return Some(execution); }}\n", Rust::expression(&function.target)));
        source.push_str(&format!("let active = {}_state(point, values)?;\nSome(Box::new(FunctionExecution::new(active)))\n", function_name(function.target)));
        source.close("}\n");
    }

    pub(super) fn write_targets(&self, source: &mut Code) {
        source.open("function_calls: data::Storage::Static(&[\n");
        for function in &self.functions {
            let shape = &function.shape;
            source.open("data::compiled::CompiledFunction {\n");
            source.push_str(&format!(
                "function: {},\n",
                Rust::expression(&function.target)
            ));
            source.open("implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {\n");
            source.push_str(&format!(
                "root: {},\nentry: {},\n",
                shape.root,
                shape.entry()
            ));
            let locals: Vec<Table<ParamLocal>> = shape
                .local_contracts()
                .into_iter()
                .map(Into::into)
                .collect();
            source.push_str(&format!("checkpoints: data::Storage::Static(&{}),\nlocals: data::Storage::Static(&{}),\ncalls: data::Storage::Static(&{}),\ncreations: data::Storage::Static(&{}),\nreturns: data::Storage::Static(&{}),\ntails: data::Storage::Static(&{}),\nstart: {}_start,\n", Rust::expression(shape.checkpoints.as_slice()), Rust::expression(locals.as_slice()), Rust::expression(shape.call_contracts().as_slice()), Rust::expression(shape.creation_contracts().as_slice()), Rust::expression(shape.return_contracts().as_slice()), Rust::expression(shape.tail_contracts().as_slice()), function_name(function.target)));
            source.close("})),\n");
            source.close("},\n");
        }
        source.close("]),\n");
    }

    fn write_continuations(&self, source: &mut Code, family: CallFamily) {
        source.open(&format!("enum {family}Return {{\n"));
        for function in &self.functions {
            for call in function
                .shape
                .calls
                .iter()
                .filter(|call| call_family(call) == family)
            {
                source.push_str(&format!(
                    "{}{},\n",
                    return_name(function.target, call.point),
                    fields(&function.shape.locals[call.point])
                ));
            }
        }
        source.close("}\n");
        let canonical = self.has_canonical_step();
        let bridge = self.has_step(family, StepKind::Bridge);
        if !canonical && !bridge && !self.has_step(family, StepKind::Return) {
            return;
        }
        source.open(&format!("impl {family}Return {{\n"));
        if canonical {
            source.open("fn site(&self) -> data::source::HostCallSite {\n");
            source.open("match *self {\n");
            for function in &self.functions {
                for call in function
                    .shape
                    .calls
                    .iter()
                    .filter(|call| call_family(call) == family)
                {
                    source.push_str(&format!(
                        "Self::{} {{ .. }} => {},\n",
                        return_name(function.target, call.point),
                        Rust::expression(&call.site)
                    ));
                }
            }
            source.close("}\n");
            source.close("}\n");
        }
        source.open(&format!(
            "fn small(self, result: {}) -> FunctionState {{\n",
            family.value_type()
        ));
        if !self.functions.iter().any(|function| {
            function
                .shape
                .calls
                .iter()
                .any(|call| call_family(call) == family)
        }) {
            source.push_str("let _ = result;\n");
        }
        source.open("match self {\n");
        for function in &self.functions {
            for call in function
                .shape
                .calls
                .iter()
                .filter(|call| call_family(call) == family)
            {
                let locals = &function.shape.locals[call.point];
                source.open(&format!(
                    "Self::{}{} => {{\n",
                    return_name(function.target, call.point),
                    pattern(locals)
                ));
                let value = match &call.output {
                    CallLocal::IntFunction { type_, .. }
                    | CallLocal::BoolFunction { type_, .. } => {
                        format!("result.with_type({})", Rust::expression(type_))
                    }
                    _ => "result".to_owned(),
                };
                source.push_str(&format!(
                    "let {} = {value};\n{}\n",
                    local_name(&call.output),
                    state(
                        function.target,
                        call.point + 1,
                        &function.shape.locals[call.point + 1]
                    )
                ));
                source.close("},\n");
            }
        }
        source.close("}\n");
        source.close("}\n");
        if !canonical && !bridge {
            source.close("}\n");
            return;
        }
        if family == CallFamily::Int {
            source.open("fn resume(self, result: CallInteger) -> FunctionState {\n");
            source.open("if let Some(result) = result.small() {\n");
            source.push_str("return self.small(result);\n");
            source.close("}\n");
            source.open("match self {\n");
            for function in &self.functions {
                for call in function
                    .shape
                    .calls
                    .iter()
                    .filter(|call| matches!(call.output, CallLocal::Int(_)))
                {
                    let locals = &function.shape.locals[call.point];
                    source.open(&format!(
                        "Self::{}{} => {{\n",
                        return_name(function.target, call.point),
                        pattern(locals)
                    ));
                    source.push_str(&format!("let {} = result;\n", local_name(&call.output)));
                    let expression = values_with_result(
                        &function.shape.locals[call.point + 1],
                        false,
                        Some(&call.output),
                    );
                    source.push_str(&format!("FunctionState::Canonical {{ target: {}, point: {}, values: {expression} }}\n", Rust::expression(&function.target), Rust::expression(&function.shape.checkpoints[call.point + 1])));
                    source.close("},\n");
                }
            }
            source.close("}\n");
            source.close("}\n");
        } else {
            source.push_str(&format!(
                "fn resume(self, result: {}) -> FunctionState {{ self.small(result) }}\n",
                family.value_type()
            ));
        }
        source.close("}\n");
    }

    fn write_function(&self, source: &mut Code, function: &CallFunction<'_, Graph>) {
        for (point, action) in function.shape.points.iter().enumerate() {
            let locals = &function.shape.locals[point];
            let checkpoint = function.shape.checkpoints[point];
            source.open(&format!(
                "FunctionState::{}{} => {{\n",
                state_name(function.target, point),
                pattern(locals)
            ));
            if let Some(numeric) = &function.numeric {
                self.write_numeric_step(source, function, numeric, point);
                source.close("},\n");
                continue;
            }
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
                    // Reading a Big head must return to the unexecuted Index,
                    // not the post-calculation overflow checkpoint.
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
                if !matches!(action, CallPoint::Tail(_)) {
                    source.push_str("*budget -= 1;\n");
                }
            }
            match action {
                CallPoint::Scalar(instruction) => {
                    write_scalar(source, instruction);
                    let outputs = match instruction {
                        CallScalar::Integer(output, _) => vec![format!("int{}", output.0)],
                        CallScalar::Region { outputs, .. } => outputs
                            .iter()
                            .map(|output| format!("int{}", output.0))
                            .collect(),
                        _ => Vec::new(),
                    };
                    if !outputs.is_empty() {
                        let checks = outputs.iter().map(|local| format!("{local} < i128::from(i64::MIN) || {local} > i128::from(i64::MAX)")).collect::<Vec<_>>().join(" || ");
                        source.push_str(&format!(
                            "if {checks} {{ return {}; }}\n",
                            canonical(
                                function.target,
                                function.shape.checkpoints[point + 1],
                                &function.shape.locals[point + 1]
                            )
                        ));
                    }
                    let next = state(
                        function.target,
                        point + 1,
                        &function.shape.locals[point + 1],
                    );
                    if let Some(index) = Self::following_return(function, point) {
                        // Preserve the Return's own charge and exact resumable
                        // point, without packaging a normal intermediate Next.
                        source.push_str(&format!(
                            "if *budget == 0 {{ return FunctionStep::Yield({next}); }}\n*budget -= 1;\n"
                        ));
                        self.write_return(source, function, index);
                    } else {
                        source.push_str(&format!("FunctionStep::Next({next})\n"));
                    }
                }
                CallPoint::Create(index) => {
                    let creation = &function.shape.creations[*index];
                    let (family, target) = match creation.target {
                        CallableTarget::Int(id) => ("int", Rust::expression(&id)),
                        CallableTarget::Bool(id) => ("bool", Rust::expression(&id)),
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
                    source.push_str(&format!("let {} = ops.{family}_{method}({target}, {}{captures});\nFunctionStep::Next({})\n", local_name(&creation.output), Rust::expression(&creation.type_), state(function.target, point + 1, &function.shape.locals[point + 1])));
                }
                CallPoint::Call(index) => {
                    self.write_call(source, function, &function.shape.calls[*index])
                }
                CallPoint::Tail(index) => self.write_tail(source, function, *index),
                CallPoint::Return(index) => {
                    self.write_return(source, function, *index);
                }
                CallPoint::Terminator(terminator) => {
                    self.write_terminator(source, function, terminator)
                }
                CallPoint::Interpreted => source.push_str(&format!(
                    "{}\n",
                    canonical(function.target, checkpoint, locals)
                )),
            }
            source.close("},\n");
        }
    }

    fn write_return(&self, source: &mut Code, function: &CallFunction<'_, Graph>, index: usize) {
        let returning = &function.shape.returns[index];
        source.push_str(&format!(
            "FunctionStep::{} {{ value: {}, exit: {} }}\n",
            target_family(function.target),
            local_expression(&returning.value, false),
            Rust::expression(&returning.exit)
        ));
    }

    fn write_tail(&self, source: &mut Code, function: &CallFunction<'_, Graph>, index: usize) {
        let tail = &function.shape.tails[index];
        let checkpoint = function.shape.checkpoints[tail.point];
        let locals = &function.shape.locals[tail.point];
        if let Some(callee) = self.static_callee(tail.target, &tail.args) {
            let parameters = &callee.shape.locals[callee.shape.entry()];
            let fields = parameters
                .iter()
                .zip(tail.args.iter())
                .map(|(parameter, argument)| field_assignment(parameter, argument))
                .collect::<Vec<_>>()
                .join(", ");
            source.push_str(&format!("FunctionStep::{}Tail {{ callee: FunctionState::{} {{ {fields} }}, completed: {}, point: {} }}\n", target_family(function.target), state_name(callee.target, callee.shape.entry()), state(function.target, tail.point, locals), Rust::expression(&checkpoint)));
            return;
        }
        source.push_str(&format!(
            "{}\n",
            canonical(function.target, checkpoint, locals)
        ));
    }

    fn write_terminator(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        terminator: &CallTerminator<'_>,
    ) {
        match terminator {
            CallTerminator::Jump(edge) => {
                source.push_str(&format!("{}\n", self.edge(function, edge)))
            }
            CallTerminator::Boolean {
                subject,
                true_,
                false_,
            } => source.push_str(&format!(
                "if bool{} {{ {} }} else {{ {} }}\n",
                subject.0,
                self.edge(function, true_),
                self.edge(function, false_)
            )),
            CallTerminator::Test {
                test,
                true_,
                false_,
            } => source.push_str(&format!(
                "if {} {{ {} }} else {{ {} }}\n",
                test_expression(test),
                self.edge(function, true_),
                self.edge(function, false_)
            )),
            CallTerminator::Switch {
                subject,
                clauses,
                fallback,
            } => {
                source.open(&format!("match int{} {{\n", subject.0));
                for (literal, edge) in *clauses {
                    source.push_str(&format!(
                        "{literal}_i128 => {},\n",
                        self.edge(function, edge)
                    ));
                }
                source.push_str(&format!("_ => {},\n", self.edge(function, fallback)));
                source.close("}\n");
            }
        }
    }

    fn edge(&self, function: &CallFunction<'_, Graph>, edge: &Edge) -> String {
        let point = function.shape.starts[&edge.target().index()];
        let fields = function.shape.locals[point]
            .iter()
            .zip(edge.args().iter().filter_map(CallLocal::inspect))
            .map(|(parameter, argument)| field_assignment(parameter, &argument))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "FunctionStep::Next(FunctionState::{} {{ {fields} }})",
            state_name(function.target, point)
        )
    }

    fn write_call(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        call: &CallInvocation,
    ) {
        let family = call_family(call);
        let prefix = &function.shape.locals[call.point];
        let caller = format!(
            "{family}Return::{}{}",
            return_name(function.target, call.point),
            pattern(prefix)
        );
        match &call.target {
            CallContractTarget::Static(target) => {
                if let Some(callee) = self.static_callee(*target, &call.args) {
                    let entry = &callee.shape.locals[callee.shape.entry()];
                    let assignments = entry
                        .iter()
                        .zip(call.args.iter())
                        .map(|(parameter, argument)| field_assignment(parameter, argument))
                        .collect::<Vec<_>>()
                        .join(", ");
                    source.push_str(&format!("FunctionStep::{family}Call {{ callee: FunctionState::{} {{ {assignments} }}, caller: {caller} }}\n", state_name(callee.target, callee.shape.entry())));
                    return;
                }
            }
            CallContractTarget::IntValue(local) => {
                source.push_str(&format!("let callable = &int_function{};\nlet captures = callable.captures();\nlet target = callable.target();\n", local.0));
                self.write_dynamic(source, call, CallFamily::Int, &caller);
            }
            CallContractTarget::BoolValue(local) => {
                source.push_str(&format!("let callable = &bool_function{};\nlet captures = callable.captures();\nlet target = callable.target();\n", local.0));
                self.write_dynamic(source, call, CallFamily::Bool, &caller);
            }
        };
        let id = match call.target {
            CallContractTarget::Static(target) => target_id(target),
            _ => "target".to_owned(),
        };
        let captures = if matches!(call.target, CallContractTarget::Static(_)) {
            "None"
        } else {
            "Some(captures.retain())"
        };
        source.push_str(&format!("FunctionStep::{family}Bridge {{ function: {id}, site: {}, arguments: CallArguments {{ values: {}, captures: {captures} }}, caller: {caller} }}\n", Rust::expression(&call.site), values(&call.args, true)));
    }

    fn write_dynamic(
        &self,
        source: &mut Code,
        call: &CallInvocation,
        family: CallFamily,
        caller: &str,
    ) {
        if let Some((index, _)) = self
            .entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.matches(family, &call.args))
        {
            let inputs = tuple(
                call.args
                    .iter()
                    .map(|argument| local_expression(argument, true)),
            );
            source.open(&format!(
                "if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_{index}(target, &captures, {inputs}) {{\n"
            ));
            source.push_str(&format!(
                "return FunctionStep::{family}Call {{ callee, caller: {caller} }};\n"
            ));
            source.close("}\n");
        }
    }

    fn write_entries(&self, source: &mut Code) {
        for (index, entry) in self.entries.iter().enumerate() {
            let callees = self
                .functions
                .iter()
                .filter(|callee| {
                    target_family(callee.target) == entry.family
                        && callee.matches_parameters(&entry.parameters)
                })
                .collect::<Vec<_>>();
            let captures = if callees.iter().all(|callee| {
                callee.shape.parameter_count == callee.shape.locals[callee.shape.entry()].len()
            }) {
                "_captures"
            } else {
                "captures"
            };
            let inputs_type = tuple(
                entry
                    .parameters
                    .iter()
                    .map(|local| local_type(local).to_owned()),
            );
            source.open(&format!("fn calls_entry_{index}(target: data::function::{}FunctionId, {captures}: &CallCaptureInputs<'_>, inputs: {inputs_type}) -> Option<FunctionState> {{\n", entry.family));
            if !entry.parameters.is_empty() {
                source.push_str(&format!(
                    "let {} = inputs;\n",
                    tuple((0..entry.parameters.len()).map(|index| format!("argument{index}")))
                ));
            } else {
                source.push_str("let () = inputs;\n");
            }
            source.open("match target.0 {\n");
            for callee in callees {
                let parameters = &callee.shape.locals[callee.shape.entry()];
                let assignments = parameters
                    .iter()
                    .take(callee.shape.parameter_count)
                    .enumerate()
                    .map(|(index, local)| format!("{}: argument{index}", local_name(local)))
                    .chain(
                        parameters
                            .iter()
                            .skip(callee.shape.parameter_count)
                            .map(|local| {
                                format!(
                                    "{}: captures.{}({})?",
                                    local_name(local),
                                    capture_method(local),
                                    local_id(local)
                                )
                            }),
                    )
                    .collect::<Vec<_>>()
                    .join(", ");
                source.push_str(&format!(
                    "{} => Some(FunctionState::{} {{ {assignments} }}),\n",
                    callee.target.index(),
                    state_name(callee.target, callee.shape.entry())
                ));
            }
            source.push_str("_ => None,\n");
            source.close("}\n");
            source.close("}\n");
        }
    }
}

impl CallableEntry {
    fn matches(&self, family: CallFamily, parameters: &[CallLocal]) -> bool {
        self.family == family
            && self.parameters.len() == parameters.len()
            && self
                .parameters
                .iter()
                .zip(parameters)
                .all(|(left, right)| left.same_type(right))
    }
}

fn return_families() -> [CallFamily; 4] {
    [
        CallFamily::Int,
        CallFamily::Bool,
        CallFamily::IntFunction,
        CallFamily::BoolFunction,
    ]
}

fn target_family(target: CallTarget) -> CallFamily {
    match target {
        CallTarget::Int(_) => CallFamily::Int,
        CallTarget::Bool(_) => CallFamily::Bool,
        CallTarget::IntFunction(_) => CallFamily::IntFunction,
        CallTarget::BoolFunction(_) => CallFamily::BoolFunction,
    }
}
fn target_id(target: CallTarget) -> String {
    match target {
        CallTarget::Int(id) => Rust::expression(&id),
        CallTarget::Bool(id) => Rust::expression(&id),
        CallTarget::IntFunction(id) => Rust::expression(&id),
        CallTarget::BoolFunction(id) => Rust::expression(&id),
    }
}
fn function_name(target: CallTarget) -> String {
    format!(
        "calls_{}_{}",
        target_family(target).name().to_lowercase(),
        target.index()
    )
}
fn state_name(target: CallTarget, point: usize) -> String {
    format!("{}{}Point{point}", target_family(target), target.index())
}
fn return_name(target: CallTarget, point: usize) -> String {
    format!("{}{}Call{point}", target_family(target), target.index())
}

fn call_family(call: &CallInvocation) -> CallFamily {
    match call.target {
        CallContractTarget::Static(target) => target_family(target),
        CallContractTarget::IntValue(_) => CallFamily::Int,
        CallContractTarget::BoolValue(_) => CallFamily::Bool,
    }
}

fn local_type(local: &CallLocal) -> &'static str {
    match local {
        CallLocal::Int(_) => "i128",
        CallLocal::Bool(_) => "bool",
        CallLocal::IntList { .. } => "IntList",
        CallLocal::IntFunction { .. } => "IntCallable",
        CallLocal::BoolFunction { .. } => "BoolCallable",
    }
}

fn local_column(local: &CallLocal) -> &'static str {
    match local {
        CallLocal::Int(_) => "ints",
        CallLocal::Bool(_) => "bools",
        CallLocal::IntList { .. } => "int_lists",
        CallLocal::IntFunction { .. } => "int_functions",
        CallLocal::BoolFunction { .. } => "bool_functions",
    }
}

fn local_name(local: &CallLocal) -> String {
    match local {
        CallLocal::Int(id) => format!("int{}", id.0),
        CallLocal::Bool(id) => format!("bool{}", id.0),
        CallLocal::IntList { local, .. } => format!("int_list{}", local.0),
        CallLocal::IntFunction { local, .. } => format!("int_function{}", local.0),
        CallLocal::BoolFunction { local, .. } => format!("bool_function{}", local.0),
    }
}
fn local_id(local: &CallLocal) -> String {
    match local {
        CallLocal::Int(id) => Rust::expression(id),
        CallLocal::Bool(id) => Rust::expression(id),
        CallLocal::IntList { local, .. } => Rust::expression(local),
        CallLocal::IntFunction { local, .. } => Rust::expression(local),
        CallLocal::BoolFunction { local, .. } => Rust::expression(local),
    }
}
fn capture_method(local: &CallLocal) -> &'static str {
    match local {
        CallLocal::Int(_) => "int",
        CallLocal::Bool(_) => "bool",
        CallLocal::IntList { .. } => "int_list",
        CallLocal::IntFunction { .. } => "int_function",
        CallLocal::BoolFunction { .. } => "bool_function",
    }
}
fn local_expression(local: &CallLocal, clone: bool) -> String {
    let name = local_name(local);
    if clone
        && matches!(
            local,
            CallLocal::IntList { .. }
                | CallLocal::IntFunction { .. }
                | CallLocal::BoolFunction { .. }
        )
    {
        format!("{name}.clone()")
    } else {
        name
    }
}
fn field_assignment(parameter: &CallLocal, argument: &CallLocal) -> String {
    let name = local_name(parameter);
    let expression = local_expression(argument, true);
    if name == expression {
        name
    } else {
        format!("{name}: {expression}")
    }
}
fn fields(locals: &[CallLocal]) -> String {
    format!(
        " {{ {} }}",
        locals
            .iter()
            .map(|local| format!("{}: {}", local_name(local), local_type(local)))
            .collect::<Vec<_>>()
            .join(", ")
    )
}
fn pattern(locals: &[CallLocal]) -> String {
    format!(
        " {{ {} }}",
        locals.iter().map(local_name).collect::<Vec<_>>().join(", ")
    )
}
fn state(target: CallTarget, point: usize, locals: &[CallLocal]) -> String {
    format!(
        "FunctionState::{}{}",
        state_name(target, point),
        pattern(locals)
    )
}

fn load_value(local: &CallLocal) -> String {
    match local {
        CallLocal::Int(id) => format!("values.int({})?", id.0),
        CallLocal::Bool(id) => format!("values.bool({})?", id.0),
        CallLocal::IntList { local, .. } => format!("values.int_list({})?", local.0),
        CallLocal::IntFunction { local, .. } => {
            format!("values.int_function({})?", local.0)
        }
        CallLocal::BoolFunction { local, .. } => {
            format!("values.bool_function({})?", local.0)
        }
    }
}

fn values(locals: &[CallLocal], clone: bool) -> String {
    values_with_result(locals, clone, None)
}

fn values_with_result(locals: &[CallLocal], clone: bool, result: Option<&CallLocal>) -> String {
    let fields = [
        "ints",
        "bools",
        "int_lists",
        "int_functions",
        "bool_functions",
    ]
    .iter()
    .map(|column| {
        let values = locals
            .iter()
            .filter(|local| local_column(local) == *column)
            .map(|local| {
                let value = local_expression(local, clone);
                if matches!(local, CallLocal::Int(_)) && result != Some(local) {
                    format!("{value}.into()")
                } else {
                    value
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("{column}: vec![{values}]")
    })
    .collect::<Vec<_>>()
    .join(", ");
    format!("CallValues {{ {fields} }}")
}

fn canonical(target: CallTarget, point: CompiledCheckpoint, locals: &[CallLocal]) -> String {
    format!(
        "FunctionStep::Canonical {{ target: {}, point: {}, values: {} }}",
        Rust::expression(&target),
        Rust::expression(&point),
        values(locals, false)
    )
}

fn capture_expression(capture: &Capture) -> String {
    match capture {
        Capture::Int { target, source } => format!(
            "CallCapture::int({}, int{})",
            Rust::expression(target),
            source.0
        ),
        Capture::Bool { target, source } => format!(
            "CallCapture::bool({}, bool{})",
            Rust::expression(target),
            source.0
        ),
        Capture::IntList { target, source } => format!(
            "CallCapture::int_list({}, int_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::IntFunction { target, source } => format!(
            "CallCapture::int_function({}, int_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::BoolFunction { target, source } => format!(
            "CallCapture::bool_function({}, bool_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
    }
}

fn operand(value: IntegerOperand) -> String {
    match value {
        IntegerOperand::Local(local) => format!("int{}", local.0),
        IntegerOperand::Immediate(value) => format!("{value}_i128"),
    }
}
fn integer_expression(expression: &NumericInteger<'_>) -> String {
    match expression {
        NumericInteger::Value(value) => format!("{value}_i128"),
        NumericInteger::Negate(local) => format!("-int{}", local.0),
        NumericInteger::Binary(operation, left, right) => {
            let left = operand(*left);
            let right = operand(*right);
            match operation {
                NumericOperation::Add => format!("{left} + {right}"),
                NumericOperation::Subtract => format!("{left} - {right}"),
                NumericOperation::Multiply => format!("{left} * {right}"),
                NumericOperation::Divide => super::division(left, right, "/"),
                NumericOperation::Remainder => super::division(left, right, "%"),
            }
        }
    }
}
fn test_expression(test: &CallTest) -> String {
    match test {
        CallTest::Not(local) => format!("!bool{}", local.0),
        CallTest::Compare(comparison, left, right) => {
            super::integer_comparison(comparison, operand(*left), operand(*right))
        }
        CallTest::IntList(test) => match test {
            IntListTest::Length {
                list,
                length,
                at_least,
            } => length_expression(&format!("int_list{}", list.0), *length, *at_least),
            IntListTest::Equal {
                left,
                right,
                negate,
            } => format!(
                "{}ops.lists().equal(&int_list{}, &int_list{})",
                if *negate { "!" } else { "" },
                left.0,
                right.0,
            ),
        },
    }
}
fn write_scalar(source: &mut Code, instruction: &CallScalar<'_>) {
    match instruction {
        CallScalar::Integer(output, value) => source.push_str(&format!(
            "let int{} = {};\n",
            output.0,
            integer_expression(value)
        )),
        CallScalar::Boolean(output, value) => {
            let value = match value {
                CallBoolean::Value(value) => value.to_string(),
                CallBoolean::Test(test) => test_expression(test),
            };
            source.push_str(&format!("let bool{} = {value};\n", output.0));
        }
        CallScalar::IntList(instruction) => match instruction {
            IntListInstruction::Value {
                output,
                type_id,
                elements,
            } => {
                let elements = elements
                    .iter()
                    .map(|local| format!("int{} as i64", local.0))
                    .collect::<Vec<_>>()
                    .join(", ");
                source.push_str(&format!(
                    "let int_list{} = ops.lists().value({}, &[{elements}]);\n",
                    output.0,
                    Rust::expression(type_id)
                ));
            }
            IntListInstruction::Spread {
                output,
                type_id,
                elements,
                tail,
            } => {
                let elements = elements
                    .iter()
                    .map(|local| format!("int{} as i64", local.0))
                    .collect::<Vec<_>>()
                    .join(", ");
                source.push_str(&format!(
                    "let int_list{} = ops.lists().prepend({}, &[{elements}], &int_list{});\n",
                    output.0,
                    Rust::expression(type_id),
                    tail.0
                ));
            }
            // The preflight has already read the head before charging Index.
            IntListInstruction::Index { .. } => {}
            IntListInstruction::Tail {
                output,
                type_id,
                list,
                count,
            } => {
                source.push_str(&format!(
                    "let int_list{} = ops.lists().tail(&int_list{}, {}, {count});\n",
                    output.0,
                    list.0,
                    Rust::expression(type_id)
                ));
            }
        },
        CallScalar::Region { region, outputs } => {
            for (index, node) in region.nodes.iter().enumerate() {
                let value = |operand| match operand {
                    ArithmeticOperand::Input(index) => format!("int{}", region.inputs[index].0),
                    ArithmeticOperand::Value(index) => format!("region{index}"),
                    ArithmeticOperand::Immediate(value) => format!("{value}_i128"),
                };
                let expression = match *node {
                    ArithmeticNode::Add(left, right) => {
                        format!("{} + {}", value(left), value(right))
                    }
                    ArithmeticNode::Subtract(left, right) => {
                        format!("{} - {}", value(left), value(right))
                    }
                    ArithmeticNode::Multiply(left, right) => {
                        format!("{} * {}", value(left), value(right))
                    }
                    ArithmeticNode::Divide(left, right) => {
                        super::division(value(left), value(right), "/")
                    }
                    ArithmeticNode::Remainder(left, right) => {
                        super::division(value(left), value(right), "%")
                    }
                    ArithmeticNode::Negate(input) => format!("-{}", value(input)),
                };
                source.push_str(&format!("let region{index} = {expression};\n"));
            }
            for (output, local) in region.outputs.iter().zip(outputs) {
                source.push_str(&format!("let int{} = region{};\n", local.0, output.value));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::shape::NumericComparison;
    use super::{
        CallBoolean, CallCodegen, CallFamily, CallLocal, CallPoint, CallScalar, CallTarget,
        CallTerminator, CallTest, Capture, Code, IntListInstruction, IntListTest, IntegerOperand,
        NumericInteger, NumericOperation, capture_expression, capture_method, fields,
        integer_expression, load_value, local_column, local_id, local_name, local_type, pattern,
        test_expression, values, write_scalar,
    };
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{
        ArithmeticNode, ArithmeticOperand, ArithmeticOutput, ArithmeticRegion, BoolFunctionLocalId,
        BoolLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId, ParamLocal, ParamSlot,
        native_proof,
    };
    use crate::plan::execution::type_::{
        FunctionType, IntListTypeId, ListTypeId, ValueShapeId, ValueType,
    };

    #[test]
    fn local_columns_and_captures_emit_each_concrete_value_family() {
        let locals = [
            CallLocal::Int(IntLocalId(0)),
            CallLocal::Bool(BoolLocalId(1)),
            CallLocal::IntList {
                local: IntListLocalId(2),
                type_id: IntListTypeId {
                    list_type: ListTypeId(0),
                },
            },
            CallLocal::IntFunction {
                local: IntFunctionLocalId(3),
                type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            },
            CallLocal::BoolFunction {
                local: BoolFunctionLocalId(4),
                type_: FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            },
        ];
        assert_eq!(
            fields(&locals),
            " { int0: i128, bool1: bool, int_list2: IntList, int_function3: IntCallable, bool_function4: BoolCallable }"
        );
        assert_eq!(
            pattern(&locals),
            " { int0, bool1, int_list2, int_function3, bool_function4 }"
        );
        assert_eq!(
            values(&locals, false),
            "CallValues { ints: vec![int0.into()], bools: vec![bool1], int_lists: vec![int_list2], int_functions: vec![int_function3], bool_functions: vec![bool_function4] }"
        );
        assert_eq!(
            values(&locals, true),
            "CallValues { ints: vec![int0.into()], bools: vec![bool1], int_lists: vec![int_list2.clone()], int_functions: vec![int_function3.clone()], bool_functions: vec![bool_function4.clone()] }"
        );
        let projections = locals
            .iter()
            .map(|local| {
                (
                    local_type(local),
                    local_column(local),
                    local_name(local),
                    local_id(local),
                    load_value(local),
                    capture_method(local),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            projections,
            [
                (
                    "i128",
                    "ints",
                    "int0".into(),
                    "data::graph::IntLocalId(0)".into(),
                    "values.int(0)?".into(),
                    "int"
                ),
                (
                    "bool",
                    "bools",
                    "bool1".into(),
                    "data::graph::BoolLocalId(1)".into(),
                    "values.bool(1)?".into(),
                    "bool"
                ),
                (
                    "IntList",
                    "int_lists",
                    "int_list2".into(),
                    "data::graph::IntListLocalId(2)".into(),
                    "values.int_list(2)?".into(),
                    "int_list"
                ),
                (
                    "IntCallable",
                    "int_functions",
                    "int_function3".into(),
                    "data::graph::IntFunctionLocalId(3)".into(),
                    "values.int_function(3)?".into(),
                    "int_function"
                ),
                (
                    "BoolCallable",
                    "bool_functions",
                    "bool_function4".into(),
                    "data::graph::BoolFunctionLocalId(4)".into(),
                    "values.bool_function(4)?".into(),
                    "bool_function"
                ),
            ]
        );
        let captures = [
            Capture::Int {
                target: IntLocalId(0),
                source: IntLocalId(7),
            },
            Capture::Bool {
                target: BoolLocalId(1),
                source: BoolLocalId(8),
            },
            Capture::IntList {
                target: IntListLocalId(2),
                source: IntListLocalId(9),
            },
            Capture::IntFunction {
                target: IntFunctionLocalId(3),
                source: IntFunctionLocalId(10),
            },
            Capture::BoolFunction {
                target: BoolFunctionLocalId(4),
                source: BoolFunctionLocalId(11),
            },
        ];
        assert_eq!(
            captures.iter().map(capture_expression).collect::<Vec<_>>(),
            [
                "CallCapture::int(data::graph::IntLocalId(0), int7)",
                "CallCapture::bool(data::graph::BoolLocalId(1), bool8)",
                "CallCapture::int_list(data::graph::IntListLocalId(2), int_list9.clone())",
                "CallCapture::int_function(data::graph::IntFunctionLocalId(3), int_function10.clone())",
                "CallCapture::bool_function(data::graph::BoolFunctionLocalId(4), bool_function11.clone())",
            ]
        );
    }

    #[test]
    fn length_only_static_list_calls_do_not_read_list_operations() {
        let input = r#"
fn identity(value: Bool) -> Bool { value }
pub fn nonempty(values: List(Int)) -> Bool {
  let empty = case values { [] -> True _ -> False }
  let result = identity(empty)
  !result
}
pub fn main() { let _ = nonempty([]) Nil }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated
                .as_str()
                .lines()
                .find(|line| line.starts_with("fn function_step(")),
            Some(
                "fn function_step(active: FunctionState, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {"
            )
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Nil
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn list_construction_and_tests_emit_concrete_typed_operations() {
        let elements = [IntLocalId(0), IntLocalId(2)];
        let mut code = Code::default();
        write_scalar(
            &mut code,
            &CallScalar::IntList(IntListInstruction::Value {
                output: IntListLocalId(3),
                type_id: IntListTypeId {
                    list_type: ListTypeId(2),
                },
                elements: &elements,
            }),
        );
        assert_eq!(
            code.as_str(),
            r#"let int_list3 = ops.lists().value(data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(2),
}, &[int0 as i64, int2 as i64]);
"#
        );
        let mut spread = Code::default();
        write_scalar(
            &mut spread,
            &CallScalar::IntList(IntListInstruction::Spread {
                output: IntListLocalId(3),
                type_id: IntListTypeId {
                    list_type: ListTypeId(2),
                },
                elements: &elements,
                tail: IntListLocalId(1),
            }),
        );
        assert_eq!(
            spread.as_str(),
            r#"let int_list3 = ops.lists().prepend(data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(2),
}, &[int0 as i64, int2 as i64], &int_list1);
"#
        );
        let mut tail = Code::default();
        write_scalar(
            &mut tail,
            &CallScalar::IntList(IntListInstruction::Tail {
                output: IntListLocalId(3),
                type_id: IntListTypeId {
                    list_type: ListTypeId(2),
                },
                list: IntListLocalId(1),
                count: 2,
            }),
        );
        assert_eq!(
            tail.as_str(),
            r#"let int_list3 = ops.lists().tail(&int_list1, data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(2),
}, 2);
"#
        );
        assert_eq!(
            test_expression(&CallTest::IntList(IntListTest::Length {
                list: IntListLocalId(3),
                length: 2,
                at_least: true,
            })),
            "int_list3.len() >= 2"
        );
        assert_eq!(
            test_expression(&CallTest::IntList(IntListTest::Equal {
                left: IntListLocalId(1),
                right: IntListLocalId(3),
                negate: false,
            })),
            "ops.lists().equal(&int_list1, &int_list3)"
        );
        assert_eq!(
            test_expression(&CallTest::IntList(IntListTest::Equal {
                left: IntListLocalId(1),
                right: IntListLocalId(3),
                negate: true,
            })),
            "!ops.lists().equal(&int_list1, &int_list3)"
        );
    }

    #[test]
    fn scalar_branches_preserve_exact_edge_values_and_static_numeric_completion() {
        let input = r#"
fn adjust(value: Int, flag: Bool) {
  let next = case flag { True -> value + 1 False -> value - 1 }
  case next { 0 -> 0 1 -> 1 _ -> case next < 0 { True -> 0 False -> next } }
}
fn forward(value: Int, flag: Bool) { adjust(value, flag) + 1 }
pub fn main() { forward(7, True) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        let adjust = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(2)))
            .unwrap();
        assert!(adjust.numeric.is_some());
        let terminators = adjust
            .shape
            .points
            .iter()
            .filter_map(|point| match point {
                CallPoint::Terminator(terminator) => Some(terminator),
                _ => None,
            })
            .map(|terminator| {
                let mut source = Code::default();
                codegen.write_terminator(&mut source, adjust, terminator);
                source.as_str().to_owned()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            terminators,
            [
                "if bool0 { FunctionStep::Next(FunctionState::Int2Point1 { int0 }) } else { FunctionStep::Next(FunctionState::Int2Point12 { int0 }) }\n",
                "FunctionStep::Next(FunctionState::Int2Point3 { int0: int1 })\n",
                "match int0 {\n    0_i128 => FunctionStep::Next(FunctionState::Int2Point4 {  }),\n    1_i128 => FunctionStep::Next(FunctionState::Int2Point6 {  }),\n    _ => FunctionStep::Next(FunctionState::Int2Point8 { int0 }),\n}\n",
                "if int0 < 0_i128 { FunctionStep::Next(FunctionState::Int2Point9 {  }) } else { FunctionStep::Next(FunctionState::Int2Point11 { int0 }) }\n",
                "FunctionStep::Next(FunctionState::Int2Point3 { int0: int1 })\n",
            ]
        );
        assert_eq!(
            generated
                .as_str()
                .lines()
                .find(|line| line.starts_with("fn function_step("))
                .unwrap(),
            "fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {"
        );
        assert_eq!(
            generated
                .as_str()
                .lines()
                .find(|line| line.contains("Self::Canonical { values"))
                .unwrap(),
            "            Self::Canonical { values, .. } => values,"
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(9.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn canonical_failure_handoff_requires_no_generated_completion_output() {
        let input = r#"
fn fail(value: Int) -> Int { echo value panic as "callee stopped" }
pub fn main() -> Int { let _ = fail(7) panic as "caller must not resume" }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated.as_str().lines().next().unwrap(),
            "use data::compiled::calls::{BoolCallable, CallArguments, CallExecution, CallInputs, CallInteger, CallOps, CallProgress, CallStorage, CallValues, IntCallable};"
        );
        let mut echo = Vec::new();
        let error = crate::run_main(&plan, &mut echo).unwrap_err();
        assert!(matches!(error, crate::ExecutionError::Panic(panic)
                if panic.message() == &crate::PanicMessage::Explicit("callee stopped".into())
                    && panic.site().function() == "fail"));
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int(7.into()));
    }

    #[test]
    fn static_identity_calls_require_no_capture_or_numeric_operations_owner() {
        let input = r#"
fn identity(value: Int) { value }
fn forward(value: Int) { identity(value) + 1 }
pub fn main() { forward(7) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated
                .as_str()
                .lines()
                .find(|line| line.starts_with("fn function_step("))
                .unwrap(),
            "fn function_step(active: FunctionState, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {"
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(8.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn later_dynamic_entry_without_a_supported_target_is_declined_exactly() {
        let source = r#"
fn unsupported(value: Int) { case [value] { [first, ..] -> first [] -> 0 } }
fn apply(calculate: fn(Int) -> Int, value: Int) {
  let next = value + 1
  calculate(next)
}
pub fn main() { apply(unsupported, 3) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let apply = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(2)))
            .unwrap();
        assert!(apply.shape.root);
        let mut generated = Code::default();
        codegen.write_start(&mut generated, apply);
        assert_eq!(
            generated.as_str(),
            r#"fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
    let active = match point {
        0 => FunctionState::Int2Point0 { int_function0: values.int_function(0)?, int0: values.int(0)? },
        1 => {
            return None
        },
        2 => FunctionState::Int2Point2 { int_function0: values.int_function(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
        _ => return None,
    };
    Some(active)
}
fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
    let active = calls_int_2_state(point, values)?;
    Some(Box::new(FunctionExecution::new(active)))
}
"#
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(4.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn boolean_only_tail_states_restore_exact_values_without_an_integer_canonical_state() {
        let source = r#"
fn predicate(flag: Bool) { !flag }
fn make(flag: Bool) { fn() { predicate(flag) } }
fn forward(flag: Bool) { make(flag) }
pub fn main() { let calculate = forward(True) calculate() }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        let values = generated
            .as_str()
            .split_once("impl FunctionState {\n")
            .unwrap()
            .1
            .split_once("enum IntReturn {\n")
            .unwrap()
            .0;
        assert_eq!(
            values,
            r#"    fn values(self) -> CallValues {
        match self {
            Self::Bool0Point0 {  } => {
                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
            },
            Self::Bool0Point1 { bool0 } => {
                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
            },
            Self::Bool0Point2 { bool0, bool_function0 } => {
                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
            },
            Self::Bool0Point3 { bool0, bool_function0, bool1 } => {
                CallValues { ints: vec![], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
            },
            Self::Bool1Point0 { bool0 } => {
                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
            },
            Self::Bool2Point0 { bool0 } => {
                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
            },
            Self::Bool2Point1 { bool0, bool1 } => {
                CallValues { ints: vec![], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
            },
            Self::BoolFunction0Point0 { bool0 } => {
                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
            },
            Self::BoolFunction1Point0 { bool0 } => {
                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
            },
            Self::BoolFunction1Point1 { bool0, bool_function0 } => {
                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
            },
        }
    }
}
"#
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Bool(false)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn scalar_call_expressions_preserve_integer_operators_and_constant_comparisons() {
        let left = IntegerOperand::Local(IntLocalId(2));
        let right = IntegerOperand::Immediate(-7);
        for (operation, expected) in [
            (NumericOperation::Add, "int2 + -7_i128"),
            (NumericOperation::Subtract, "int2 - -7_i128"),
            (NumericOperation::Multiply, "int2 * -7_i128"),
            (
                NumericOperation::Divide,
                "if -7_i128 == 0 { 0_i128 } else { int2 / -7_i128 }",
            ),
            (
                NumericOperation::Remainder,
                "if -7_i128 == 0 { 0_i128 } else { int2 % -7_i128 }",
            ),
        ] {
            assert_eq!(
                integer_expression(&NumericInteger::Binary(operation, left, right)),
                expected
            );
        }
        for right in [-1, 0, 1] {
            assert_eq!(
                integer_expression(&NumericInteger::Binary(
                    NumericOperation::Remainder,
                    left,
                    IntegerOperand::Immediate(right)
                )),
                "0_i128"
            );
        }
        for (comparison, expression) in [
            (NumericComparison::Equal, "int2 == -7_i128"),
            (NumericComparison::NotEqual, "int2 != -7_i128"),
            (NumericComparison::Less, "int2 < -7_i128"),
            (NumericComparison::LessEqual, "int2 <= -7_i128"),
            (NumericComparison::Greater, "int2 > -7_i128"),
            (NumericComparison::GreaterEqual, "int2 >= -7_i128"),
        ] {
            assert_eq!(
                test_expression(&CallTest::Compare(comparison, left, right)),
                expression
            );
        }
        for operand in [left, right] {
            for (comparison, reflexive) in [
                (NumericComparison::Equal, "true"),
                (NumericComparison::NotEqual, "false"),
                (NumericComparison::Less, "false"),
                (NumericComparison::LessEqual, "true"),
                (NumericComparison::Greater, "false"),
                (NumericComparison::GreaterEqual, "true"),
            ] {
                assert_eq!(
                    test_expression(&CallTest::Compare(comparison, operand, operand)),
                    reflexive
                );
            }
        }
        assert_eq!(test_expression(&CallTest::Not(BoolLocalId(5))), "!bool5");
        assert_eq!(
            integer_expression(&NumericInteger::Negate(IntLocalId(2))),
            "-int2"
        );
    }

    #[test]
    fn native_region_call_emission_preserves_each_integer_node_and_completed_output() {
        let region = ArithmeticRegion {
            inputs: vec![IntLocalId(0), IntLocalId(1)].into(),
            nodes: vec![
                ArithmeticNode::Add(ArithmeticOperand::Input(0), ArithmeticOperand::Immediate(3)),
                ArithmeticNode::Subtract(
                    ArithmeticOperand::Value(0),
                    ArithmeticOperand::Immediate(7),
                ),
                ArithmeticNode::Multiply(
                    ArithmeticOperand::Value(1),
                    ArithmeticOperand::Immediate(2),
                ),
                ArithmeticNode::Divide(ArithmeticOperand::Value(2), ArithmeticOperand::Input(1)),
                ArithmeticNode::Remainder(ArithmeticOperand::Value(3), ArithmeticOperand::Input(1)),
                ArithmeticNode::Negate(ArithmeticOperand::Value(4)),
            ]
            .into(),
            outputs: vec![
                ArithmeticOutput {
                    value: 0,
                    slot: ParamSlot::new(ParamLocal::Int(IntLocalId(2)), ValueShapeId(0)),
                },
                ArithmeticOutput {
                    value: 5,
                    slot: ParamSlot::new(ParamLocal::Int(IntLocalId(3)), ValueShapeId(0)),
                },
            ]
            .into(),
            native: true,
        };
        assert!(native_proof(2, &region.nodes));
        let mut source = Code::default();
        write_scalar(
            &mut source,
            &CallScalar::Region {
                region: &region,
                outputs: vec![IntLocalId(2), IntLocalId(3)],
            },
        );
        assert_eq!(
            source.as_str(),
            r#"let region0 = int0 + 3_i128;
let region1 = region0 - 7_i128;
let region2 = region1 * 2_i128;
let region3 = if int1 == 0 { 0_i128 } else { region2 / int1 };
let region4 = if int1 == 0 { 0_i128 } else { region3 % int1 };
let region5 = -region4;
let int2 = region0;
let int3 = region5;
"#
        );
    }

    #[test]
    fn every_generated_return_family_has_its_exact_rust_name() {
        for (family, expected) in [
            (CallFamily::Int, "Int"),
            (CallFamily::Bool, "Bool"),
            (CallFamily::IntFunction, "IntFunction"),
            (CallFamily::BoolFunction, "BoolFunction"),
        ] {
            assert_eq!(family.to_string(), expected);
        }
    }

    #[test]
    fn interpreted_boolean_bridge_advances_once_without_mutating_the_active_local() {
        let input = r#"
pub fn verify() -> Bool {
  let assert True = accepted()
  accepted()
}
fn accepted() -> Bool {
  let value = "ok"
  value == "ok"
}
pub fn main() { let _ = verify() Nil }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut protocol = Code::default();
        codegen.write_protocol(&mut protocol);
        assert_eq!(
            protocol.as_str(),
            r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Yield(FunctionState),
    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
    BoolBridge { function: data::function::BoolFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: BoolReturn },
}
"#
        );
        let mut advance = Code::default();
        codegen.write_advance(&mut advance);
        assert_eq!(
            advance.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
    match function_step(active, ops, budget) {
        FunctionStep::Yield(active) => {
            self.active = Some(active);
            CallProgress::Yield(self)
        },
        FunctionStep::BoolBridge { function, site, arguments, caller } => CallProgress::Bool {
            function, site, arguments,
            resume: Box::new(move |value| {
                self.active = Some(caller.resume(value));
                self
            }),
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
                    CallProgress::Interpreted { point, values }
                },
                data::compiled::CallTarget::Bool(function) => {
                    if let Some(caller) = self.boolean_returns.pop() {
                        let site = caller.site();
                        return CallProgress::InterpretedBool {
                            function, site, point, values,
                            resume: Box::new(move |value| {
                                self.active = Some(caller.resume(value));
                                self
                            }),
                        };
                    }
                    CallProgress::Interpreted { point, values }
                },
                data::compiled::CallTarget::IntFunction(function) => {
                    if let Some(caller) = self.integer_function_returns.pop() {
                        let site = caller.site();
                        return CallProgress::InterpretedIntFunction {
                            function, site, point, values,
                            resume: Box::new(move |value| {
                                self.active = Some(caller.resume(value));
                                self
                            }),
                        };
                    }
                    CallProgress::Interpreted { point, values }
                },
                data::compiled::CallTarget::BoolFunction(function) => {
                    if let Some(caller) = self.boolean_function_returns.pop() {
                        let site = caller.site();
                        return CallProgress::InterpretedBoolFunction {
                            function, site, point, values,
                            resume: Box::new(move |value| {
                                self.active = Some(caller.resume(value));
                                self
                            }),
                        };
                    }
                    CallProgress::Interpreted { point, values }
                },
            }
        },
    }
}
"#
        );
    }

    #[test]
    fn boolean_only_protocol_keeps_yields_and_direct_returns_without_unused_steps() {
        let input = r#"
fn identity(value: Bool) -> Bool { value }
pub fn flip(value: Bool) -> Bool {
  let result = identity(value)
  !result
}
pub fn main() { let _ = flip(False) Nil }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut protocol = Code::default();
        codegen.write_protocol(&mut protocol);
        assert_eq!(
            protocol.as_str(),
            r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Yield(FunctionState),
    BoolCall { callee: FunctionState, caller: BoolReturn },
    Bool { value: bool, exit: data::graph::BlockGraphExitId },
}
"#
        );
        let flip = &codegen.functions[0];
        let mut steps = Code::default();
        codegen.write_function(&mut steps, flip);
        assert_eq!(
            steps.as_str(),
            r#"FunctionState::Bool0Point0 { bool0 } => {
    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { bool0 }); }
    *budget -= 1;
    FunctionStep::BoolCall { callee: FunctionState::Bool1Point0 { bool0 }, caller: BoolReturn::Bool0Call0 { bool0 } }
},
FunctionState::Bool0Point1 { bool0, bool1 } => {
    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { bool0, bool1 }); }
    *budget -= 1;
    let bool2 = !bool1;
    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { bool0, bool1, bool2 }); }
    *budget -= 1;
    FunctionStep::Bool { value: bool2, exit: data::graph::BlockGraphExitId(0) }
},
FunctionState::Bool0Point2 { bool0, bool1, bool2 } => {
    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { bool0, bool1, bool2 }); }
    *budget -= 1;
    FunctionStep::Bool { value: bool2, exit: data::graph::BlockGraphExitId(0) }
},
"#
        );
        let mut execution = Code::default();
        codegen.write_advance(&mut execution);
        assert_eq!(
            execution.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
    loop {
        match function_step(active, ops, budget) {
            FunctionStep::Yield(active) => {
                self.active = Some(active);
                return CallProgress::Yield(self);
            },
            FunctionStep::BoolCall { callee, caller } => {
                self.boolean_returns.push(caller);
                active = callee;
            },
            FunctionStep::Bool { value, exit } => {
                if let Some(caller) = self.boolean_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { exit, output: CallOutput::Bool(value), execution: self };
                }
            },
        }
    }
}
"#
        );
        let mut continuation = Code::default();
        codegen.write_continuations(&mut continuation, CallFamily::Bool);
        assert_eq!(
            continuation.as_str(),
            r#"enum BoolReturn {
    Bool0Call0 { bool0: bool },
}
impl BoolReturn {
    fn small(self, result: bool) -> FunctionState {
        match self {
            Self::Bool0Call0 { bool0 } => {
                let bool1 = result;
                FunctionState::Bool0Point1 { bool0, bool1 }
            },
        }
    }
}
"#
        );
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated.as_str().lines().next(),
            Some(
                "use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage};"
            )
        );
    }

    #[test]
    fn arithmetic_outputs_keep_the_canonical_protocol_without_integer_calls() {
        let input = r#"
fn predicate(value: Bool) -> Bool { !value }
pub fn compare_sum(left: Int, right: Int) -> Bool {
  let sum = left + right + left
  let result = predicate(sum > right)
  !result
}
pub fn main() { let _ = compare_sum(4, 5) Nil }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let arithmetic = codegen.functions[0]
            .shape
            .points
            .iter()
            .filter_map(|point| match point {
                CallPoint::Scalar(CallScalar::Region { region, outputs }) => Some((
                    region.inputs.as_ref(),
                    region.nodes.as_ref(),
                    outputs.as_slice(),
                    region.native,
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            arithmetic.as_slice(),
            [(
                &[IntLocalId(0), IntLocalId(1)][..],
                &[
                    ArithmeticNode::Add(ArithmeticOperand::Input(0), ArithmeticOperand::Input(1),),
                    ArithmeticNode::Add(ArithmeticOperand::Value(0), ArithmeticOperand::Input(0),),
                ][..],
                &[IntLocalId(2)][..],
                true,
            )]
        );
        let mut protocol = Code::default();
        codegen.write_protocol(&mut protocol);
        assert_eq!(
            protocol.as_str(),
            r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Next(FunctionState),
    Yield(FunctionState),
    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
    BoolCall { callee: FunctionState, caller: BoolReturn },
    Bool { value: bool, exit: data::graph::BlockGraphExitId },
}
"#
        );
        let mut steps = Code::default();
        codegen.write_function(&mut steps, &codegen.functions[0]);
        assert!(steps.as_str().contains(
            "if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical"
        ));
    }

    #[test]
    fn list_index_keeps_the_canonical_protocol_without_integer_calls_or_arithmetic() {
        let input = r#"
fn inspect(value: Bool) -> Bool { !value }
pub fn first_greater(values: List(Int), minimum: Int) -> Bool {
  let greater = case values {
    [head, ..] -> head > minimum
    [] -> False
  }
  let result = inspect(greater)
  !result
}
pub fn main() { let _ = first_greater([4], 3) Nil }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        assert_eq!(
            codegen
                .functions
                .iter()
                .map(|function| function.numeric.is_some())
                .collect::<Vec<_>>(),
            [false, false]
        );
        assert_eq!(
            codegen
                .functions
                .iter()
                .flat_map(|function| &function.shape.calls)
                .map(|call| matches!(call.output, CallLocal::Int(_)))
                .collect::<Vec<_>>(),
            [false]
        );
        let indices = codegen
            .functions
            .iter()
            .flat_map(|function| &function.shape.points)
            .filter_map(|point| match point {
                CallPoint::Scalar(CallScalar::IntList(IntListInstruction::Index {
                    output,
                    list,
                    index,
                })) => Some((*output, *list, *index)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(indices, [(IntLocalId(1), IntListLocalId(0), 0)]);
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated
                .as_str()
                .lines()
                .find(|line| line.starts_with("fn function_step(")),
            Some(
                "fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {"
            )
        );
        let mut protocol = Code::default();
        codegen.write_protocol(&mut protocol);
        assert_eq!(
            protocol.as_str(),
            r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Next(FunctionState),
    Yield(FunctionState),
    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
    BoolCall { callee: FunctionState, caller: BoolReturn },
    Bool { value: bool, exit: data::graph::BlockGraphExitId },
}
"#
        );
    }

    #[test]
    fn list_equality_keeps_operations_for_values_and_guards() {
        for (input, expected_kind) in [
            (
                r#"
fn inspect(value: Bool) -> Bool { !value }
pub fn same(values: List(Int), expected: List(Int)) -> Bool {
  let equal = values == expected
  let checked = inspect(equal)
  !checked
}
pub fn main() { let _ = same([1], [1]) Nil }
"#,
                "value",
            ),
            (
                r#"
fn inspect(value: Bool) -> Bool { !value }
pub fn same(values: List(Int), expected: List(Int)) -> Bool {
  let equal = case values {
    contents if contents == expected -> True
    _ -> False
  }
  let checked = inspect(equal)
  !checked
}
pub fn main() { let _ = same([1], [1]) Nil }
"#,
                "guard",
            ),
        ] {
            let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let codegen = CallCodegen::new(&plan.program.functions);
            assert!(codegen.entries.is_empty());
            assert_eq!(
                codegen
                    .functions
                    .iter()
                    .map(|function| (function.numeric.is_some(), function.shape.creations.len()))
                    .collect::<Vec<_>>(),
                [(false, 0), (false, 0)]
            );
            let comparisons = codegen
                .functions
                .iter()
                .flat_map(|function| &function.shape.points)
                .filter_map(|point| match point {
                    CallPoint::Scalar(CallScalar::Boolean(
                        _,
                        CallBoolean::Test(test @ CallTest::IntList(IntListTest::Equal { .. })),
                    )) => Some(("value", test_expression(test))),
                    CallPoint::Terminator(CallTerminator::Test {
                        test: test @ CallTest::IntList(IntListTest::Equal { .. }),
                        ..
                    }) => Some(("guard", test_expression(test))),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                comparisons,
                [(
                    expected_kind,
                    "ops.lists().equal(&int_list0, &int_list1)".into()
                )]
            );
            let mut generated = Code::default();
            codegen.write_code(&mut generated);
            assert_eq!(
                generated
                    .as_str()
                    .lines()
                    .find(|line| line.starts_with("fn function_step(")),
                Some(
                    "fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {"
                )
            );
            assert!(
                generated
                    .as_str()
                    .contains("ops.lists().equal(&int_list0, &int_list1)")
            );
        }
    }

    #[test]
    fn normal_completion_protocol_has_exact_direct_results_for_all_four_return_families() {
        let input = r#"
fn integer(value: Int) -> Int { value }
fn boolean(value: Bool) -> Bool { value }
fn integer_function() -> fn(Int) -> Int { integer }
fn boolean_function() -> fn(Bool) -> Bool { boolean }
pub fn main() { #(integer_function()(7), boolean_function()(True)) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let mut source = Code::default();
        codegen.write_protocol(&mut source);
        assert_eq!(
            source.as_str(),
            r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Next(FunctionState),
    Yield(FunctionState),
    Int { value: i128, exit: data::graph::BlockGraphExitId },
    Bool { value: bool, exit: data::graph::BlockGraphExitId },
    IntFunction { value: IntCallable, exit: data::graph::BlockGraphExitId },
    BoolFunction { value: BoolCallable, exit: data::graph::BlockGraphExitId },
}
"#
        );
        let mut advance = Code::default();
        codegen.write_advance(&mut advance);
        assert_eq!(
            advance.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
    loop {
        match function_step(active, ops, budget) {
            FunctionStep::Next(next) => active = next,
            FunctionStep::Yield(active) => {
                self.active = Some(active);
                return CallProgress::Yield(self);
            },
            FunctionStep::Int { value, exit } => {
                if let Some(caller) = self.integer_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { exit, output: CallOutput::Int(value.into()), execution: self };
                }
            },
            FunctionStep::Bool { value, exit } => {
                if let Some(caller) = self.boolean_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { exit, output: CallOutput::Bool(value), execution: self };
                }
            },
            FunctionStep::IntFunction { value, exit } => {
                if let Some(caller) = self.integer_function_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { exit, output: CallOutput::IntFunction(value), execution: self };
                }
            },
            FunctionStep::BoolFunction { value, exit } => {
                if let Some(caller) = self.boolean_function_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { exit, output: CallOutput::BoolFunction(value), execution: self };
                }
            },
        }
    }
}
"#
        );
        let mut complete = Code::default();
        codegen.write_code(&mut complete);
        assert!(!complete.as_str().contains("fn values(self) -> CallValues"));
    }

    #[test]
    fn identity_has_exact_typed_entry_return_and_zero_budget_checkpoint() {
        let source = "fn identity(value: Int) { value } pub fn main() { let calculate = identity calculate(7) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let identity = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(1)))
            .unwrap();
        assert!(identity.numeric.is_none());
        let mut source = Code::default();
        codegen.write_start(&mut source, identity);
        assert_eq!(
            source.as_str(),
            r#"fn calls_int_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
    let active = match point {
        0 => FunctionState::Int1Point0 { int0: values.int(0)? },
        _ => return None,
    };
    Some(active)
}
fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
    let active = calls_int_1_state(point, values)?;
    Some(Box::new(FunctionExecution::new(active)))
}
"#
        );
        let mut source = Code::default();
        codegen.write_function(&mut source, identity);
        assert_eq!(
            source.as_str(),
            r#"FunctionState::Int1Point0 { int0 } => {
    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { int0 }); }
    *budget -= 1;
    FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
},
"#
        );
    }

    #[test]
    fn zero_argument_forwarding_has_no_unused_state_input() {
        let input = r#"
fn identity(value: Int) -> Int { value }
fn stop() -> Int { panic as "stopped" }
fn fail() -> Int { stop() }
pub fn main() -> Int { let calculate = identity let _ = calculate(7) fail() }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let forward = codegen
            .functions
            .iter()
            .find(|function| function.shape.locals.iter().all(Vec::is_empty))
            .unwrap();
        let mut source = Code::default();
        codegen.write_start(&mut source, forward);
        assert_eq!(
            source.as_str(),
            r#"fn calls_int_2_state(point: usize, _values: CallInputs<'_>) -> Option<FunctionState> {
    let active = match point {
        0 => FunctionState::Int2Point0 {  },
        _ => return None,
    };
    Some(active)
}
fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
    let active = calls_int_2_state(point, values)?;
    Some(Box::new(FunctionExecution::new(active)))
}
"#
        );
        let mut echo = Vec::new();
        let error = crate::run_main(&plan, &mut echo).unwrap_err();
        assert!(matches!(error, crate::ExecutionError::Panic(panic)
            if panic.message() == &crate::PanicMessage::Explicit("stopped".into())
                && panic.site().function() == "stop"));
        assert!(echo.is_empty());
    }

    #[test]
    fn repeated_callable_invocations_share_one_exact_target_and_capture_entry() {
        let source = "fn identity(value: Int) { value } pub fn main() { let calculate = identity calculate(7) + calculate(8) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        assert_eq!(codegen.entries.len(), 1);
        assert_eq!(
            codegen
                .functions
                .iter()
                .map(|function| function.shape.calls.len())
                .sum::<usize>(),
            2
        );
        let mut source = Code::default();
        codegen.write_entries(&mut source);
        assert_eq!(
            source.as_str(),
            r#"fn calls_entry_0(target: data::function::IntFunctionId, _captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
    let (argument0,) = inputs;
    match target.0 {
        1 => Some(FunctionState::Int1Point0 { int0: argument0 }),
        _ => None,
    }
}
"#
        );
    }

    #[test]
    fn root_entry_checks_the_actual_target_before_constructing_owned_call_state() {
        let source = r#"
fn identity(value: Int) { value }
fn apply(calculate: fn(Int) -> Int, value: Int) { calculate(value) }
pub fn main() { apply(identity, 7) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(&plan.program.functions);
        let main = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        // main first specializes the identity argument, then its apply target.
        assert_eq!(
            main.shape.tails[0].target,
            CallTarget::Int(IntFunctionId(2))
        );
        let apply = codegen
            .functions
            .iter()
            .find(|function| function.target == main.shape.tails[0].target)
            .unwrap();
        assert!(apply.shape.root);
        let mut source = Code::default();
        codegen.write_start(&mut source, apply);
        assert_eq!(
            source.as_str(),
            r#"fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
    let active = match point {
        0 => {
            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(1)) { return None; }
            FunctionState::Int2Point0 { int_function0: values.int_function(0)?, int0: values.int(0)? }
        },
        1 => FunctionState::Int2Point1 { int_function0: values.int_function(0)?, int0: values.int(0)?, int1: values.int(1)? },
        _ => return None,
    };
    Some(active)
}
fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
    let active = calls_int_2_state(point, values)?;
    Some(Box::new(FunctionExecution::new(active)))
}
"#
        );
    }
}
