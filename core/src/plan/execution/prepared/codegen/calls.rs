mod flow;
mod group;
mod kernel;
mod local;
mod matching;
mod native;
mod nullary;
mod scalar_native;

use self::group::CallGroup;
use self::local::{
    capture_input_expression, field_assignment, fields, load_value, local_expression, local_name,
    local_type, pattern, values, values_with_result,
};
pub(in crate::plan::execution::prepared) mod shape;

use self::shape::{
    CallBoolean, CallFunction, CallInvocation, CallLocal, CallPoint, CallProgram, CallScalar,
    CallTerminator, CallTest, FloatComparison, FloatOperation, PrimitiveListLocal,
    PrimitiveListOperation,
};
use super::int_list::{IntListInstruction, IntListTest};
use super::shape::{NumericInteger, NumericOperation};
use super::string::StringOperation;
use super::{Code, CompiledShape, length_expression, tuple};
use crate::plan::execution::compiled::{CallContractTarget, CallTarget, CompiledCheckpoint};
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionGraphProfile, ExecutionProfile,
    FunctionTables,
};
use crate::plan::execution::graph::{
    ArithmeticNode, ArithmeticOperand, Edge, IntegerOperand, ParamLocal,
};
use crate::plan::execution::prepared::rust::Rust;
use crate::plan::execution::storage::Table;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub(in crate::plan::execution::prepared) struct CallCodegen<'graph, Graph: ExecutionGraphProfile> {
    functions: Vec<CallFunction<'graph, Graph>>,
    groups: Vec<CallGroup>,
    native_targets: BTreeSet<(usize, usize)>,
}

struct CallGroupCodegen<'codegen, 'graph, Graph: ExecutionGraphProfile> {
    functions: Vec<&'codegen CallFunction<'graph, Graph>>,
    entry_targets: Vec<CallTarget>,
    entries: Vec<CallableEntry>,
    native_targets: BTreeSet<(usize, usize)>,
}

struct CallableEntry {
    family: CallFamily,
    parameters: Vec<CallLocal>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CallFamily {
    Int,
    Bool,
    IntFunction,
    BoolFunction,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Nil,
    FloatFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    NilFunction,
    Custom,
    Tuple,
}

impl CallFamily {
    fn name(self) -> &'static str {
        match self {
            Self::Custom => "Custom",
            Self::Tuple => "Tuple",
            Self::Int => "Int",
            Self::Bool => "Bool",
            Self::IntFunction => "IntFunction",
            Self::BoolFunction => "BoolFunction",
            Self::Float => "Float",
            Self::String => "String",
            Self::BitArray => "BitArray",
            Self::UtfCodepoint => "UtfCodepoint",
            Self::Nil => "Nil",
            Self::FloatFunction => "FloatFunction",
            Self::StringFunction => "StringFunction",
            Self::BitArrayFunction => "BitArrayFunction",
            Self::UtfCodepointFunction => "UtfCodepointFunction",
            Self::NilFunction => "NilFunction",
        }
    }
    fn return_stack(self) -> &'static str {
        match self {
            Self::Custom => "custom_returns",
            Self::Tuple => "tuple_returns",
            Self::Int => "integer_returns",
            Self::Bool => "boolean_returns",
            Self::IntFunction => "integer_function_returns",
            Self::BoolFunction => "boolean_function_returns",
            Self::Float => "float_returns",
            Self::String => "string_returns",
            Self::BitArray => "bit_array_returns",
            Self::UtfCodepoint => "utf_codepoint_returns",
            Self::Nil => "nil_returns",
            Self::FloatFunction => "float_function_returns",
            Self::StringFunction => "string_function_returns",
            Self::BitArrayFunction => "bit_array_function_returns",
            Self::UtfCodepointFunction => "utf_codepoint_function_returns",
            Self::NilFunction => "nil_function_returns",
        }
    }
    fn value_type(self) -> &'static str {
        match self {
            Self::Custom => "CallCustom",
            Self::Tuple => "CallTuple",
            Self::Int => "i128",
            Self::Bool => "bool",
            Self::IntFunction => "IntCallable",
            Self::BoolFunction => "BoolCallable",
            Self::Float => "f64",
            Self::String => "StringValue",
            Self::BitArray => "CallBitArray",
            Self::UtfCodepoint => "char",
            Self::Nil => "()",
            Self::FloatFunction => "FloatCallable",
            Self::StringFunction => "StringCallable",
            Self::BitArrayFunction => "BitArrayCallable",
            Self::UtfCodepointFunction => "UtfCodepointCallable",
            Self::NilFunction => "NilCallable",
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
        custom_types: &crate::plan::execution::type_::CustomTypeTable,
        value_shapes: &crate::plan::execution::type_::ValueShapeTable,
    ) -> Self {
        let mut native_targets: BTreeSet<_> = functions
            .value_returns
            .string_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                matches!(function.as_ref(), ExecutionFunctionRef::Host(_)).then_some(
                    CallTarget::String(crate::plan::execution::function::StringFunctionId(index))
                        .key(),
                )
            })
            .collect();
        for (index, function) in functions.value_returns.custom_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Host(_) = function.as_ref() {
                native_targets.insert((14, index));
            }
        }
        for (index, function) in functions.value_returns.tuple_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Host(_) = function.as_ref() {
                native_targets.insert((15, index));
            }
        }
        let functions = CallProgram::inspect(functions, custom_types, value_shapes).functions;
        let groups = CallGroup::inspect(&functions);
        Self {
            functions,
            groups,
            native_targets,
        }
    }

    pub(super) fn is_root(&self, target: CallTarget) -> bool {
        self.functions
            .iter()
            .any(|function| function.target == target && function.shape.root)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    pub(super) fn write_code(&self, source: &mut Code) {
        for (index, group) in self.groups.iter().enumerate() {
            source.open(&format!(
                "const CALL_GROUP_{index}: [data::compiled::calls::CallStart; {}] = {{\n",
                group.entries.len()
            ));
            let codegen = CallGroupCodegen::new(
                group
                    .members
                    .iter()
                    .map(|&member| &self.functions[member])
                    .collect(),
                group
                    .entries
                    .iter()
                    .map(|&entry| self.functions[entry].target)
                    .collect(),
                self.native_targets.clone(),
            );
            codegen.write_code(source);
            let starts = group
                .entries
                .iter()
                .map(|&entry| format!("{}_start", function_name(self.functions[entry].target)))
                .collect::<Vec<_>>()
                .join(", ");
            source.push_str(&format!("[{starts}]\n"));
            source.close("};\n");
        }
    }

    pub(super) fn write_targets(&self, source: &mut Code) {
        source.open("function_calls: data::Storage::Static(&[\n");
        let entries: BTreeMap<_, _> = self
            .groups
            .iter()
            .enumerate()
            .flat_map(|(group, members)| {
                members
                    .entries
                    .iter()
                    .enumerate()
                    .map(move |(slot, &entry)| (entry, (group, slot)))
            })
            .collect();
        for (entry, (group, slot)) in entries {
            let function = &self.functions[entry];
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
            source.push_str(&format!("checkpoints: data::Storage::Static(&{}),\nlocals: data::Storage::Static(&{}),\ncalls: data::Storage::Static(&{}),\ncreations: data::Storage::Static(&{}),\nreturns: data::Storage::Static(&{}),\ntails: data::Storage::Static(&{}),\nstart: CALL_GROUP_{group}[{slot}],\n", Rust::expression(shape.checkpoints.as_slice()), Rust::expression(locals.as_slice()), Rust::expression(shape.call_contracts().as_slice()), Rust::expression(shape.creation_contracts().as_slice()), Rust::expression(shape.return_contracts().as_slice()), Rust::expression(shape.tail_contracts().as_slice())));
            source.close("})),\n");
            source.close("},\n");
        }
        source.close("]),\n");
    }
}

impl<'codegen, 'graph, Graph: ExecutionGraphProfile> CallGroupCodegen<'codegen, 'graph, Graph> {
    fn new(
        functions: Vec<&'codegen CallFunction<'graph, Graph>>,
        entry_targets: Vec<CallTarget>,
        native_targets: BTreeSet<(usize, usize)>,
    ) -> Self {
        let mut codegen = Self {
            functions,
            entry_targets,
            entries: Vec::new(),
            native_targets,
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
                    .any(|callee| !callee.native_loop && callee.accepts_call(call))
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

    pub(super) fn write_code(&self, source: &mut Code) {
        let mut imports = vec![
            "CallExecution",
            "CallInputs",
            "CallOps",
            "CallProgress",
            "CallStorage",
        ];
        let canonical = self.has_canonical_step();
        let native = self.has_native();
        // Imports follow emitted fields and entry signatures. A scalar segment
        // can create a callable and transfer it directly to the canonical owner
        // without declaring any generated field of that callable's type.
        let mut field_types = BTreeSet::new();
        for function in &self.functions {
            for (point, locals) in function.shape.locals.iter().enumerate() {
                if self.local_point(function, point) {
                    field_types.extend(locals.iter().map(local_type));
                }
            }
            for call in &function.shape.calls {
                field_types.extend(function.shape.locals[call.point].iter().map(local_type));
            }
        }
        for entry in &self.entries {
            field_types.extend(entry.parameters.iter().map(local_type));
        }
        for family in self.return_families() {
            if canonical
                || self.has_step(family, StepKind::Bridge)
                || self.has_step(family, StepKind::Return)
            {
                field_types.insert(family.value_type());
            }
        }
        if native {
            imports.extend(["CallValues", "CallOutput"]);
            for (family, execution, request) in [
                (
                    CallFamily::String,
                    "StringNativeExecution",
                    "StringNativeRequest",
                ),
                (
                    CallFamily::Custom,
                    "CustomNativeExecution",
                    "CustomNativeRequest",
                ),
                (
                    CallFamily::Tuple,
                    "TupleNativeExecution",
                    "TupleNativeRequest",
                ),
            ] {
                if self.native_families().contains(&family) {
                    imports.extend([execution, request, family.value_type()]);
                }
            }
        }
        for name in [
            "IntCallable",
            "BoolCallable",
            "FloatCallable",
            "StringCallable",
            "BitArrayCallable",
            "UtfCodepointCallable",
            "NilCallable",
            "StringValue",
            "CallBitArray",
            "CallNullary",
            "CallCustom",
            "CallTuple",
        ] {
            if field_types.contains(name) {
                imports.push(name);
            }
        }
        let native_bridge = self.has_native_calls();
        if native_bridge {
            imports.extend([
                "CallNativeFailure",
                "CallNativeInput",
                "CallNativeOps",
                "CallNativeReturn",
            ]);
        }
        if (canonical && self.return_families().contains(&CallFamily::Int))
            || self.has_native_bridge(CallFamily::Int)
        {
            imports.push("CallInteger");
        }
        if canonical
            || self
                .return_families()
                .iter()
                .any(|family| self.has_step(*family, StepKind::Bridge))
        {
            imports.push("CallValues");
        }
        if self
            .return_families()
            .iter()
            .any(|family| self.has_step(*family, StepKind::Return))
        {
            imports.push("CallOutput");
        }
        if self
            .return_families()
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
        imports.dedup();
        source.push_str(&format!(
            "use data::compiled::calls::{{{}}};\n",
            imports.join(", ")
        ));
        if field_types.contains("IntList") {
            source.push_str("use data::compiled::int_list::IntList;\n");
        }
        for name in [
            "BoolList",
            "FloatList",
            "StringList",
            "BitArrayList",
            "UtfCodepointList",
            "NilList",
        ] {
            if field_types.contains(name) {
                source.push_str(&format!("use data::compiled::primitive_list::{name};\n"));
            }
        }
        let canonical_return = self.functions.iter().any(|function| {
            function
                .shape
                .calls
                .iter()
                .any(|call| matches!(call.output, CallLocal::Int(_) | CallLocal::Nullary(_)))
        });
        source.open("enum FunctionState {\n");
        for function in &self.functions {
            for (point, locals) in function
                .shape
                .locals
                .iter()
                .enumerate()
                .filter(|(point, _)| self.global_point(function, *point))
            {
                source.push_str(&format!(
                    "{}{},\n",
                    state_name(function.target, point),
                    fields(locals)
                ));
            }
            self.write_kernel_state(source, function);
        }
        for family in self.native_families() {
            source.push_str(&format!(
                "{family}NativeComplete {{ value: {} }},\n",
                family.value_type()
            ));
        }
        if canonical_return {
            source.push_str("Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },\n");
        }
        source.close("}\n");
        for family in self.return_families() {
            self.write_continuations(source, family);
        }
        self.write_protocol(source);
        self.write_execution(source);
        source.open("fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {\n");
        source.open("match active {\n");
        if canonical_return {
            source.push_str("FunctionState::Canonical { target, point, values } => FunctionStep::Canonical { target, point, values },\n");
        }
        for family in self.native_families() {
            source.push_str(&format!("FunctionState::{family}NativeComplete {{ value }} => FunctionStep::{family}NativeComplete {{ value }},\n"));
        }
        for function in &self.functions {
            self.write_function(source, function);
            self.write_kernel_resume(source, function);
        }
        source.close("}\n");
        source.close("}\n");
        self.write_kernel_bodies(source);
        for function in &self.functions {
            self.write_body(source, function);
        }
        self.write_entries(source);
        for function in &self.functions {
            if let Some(numeric) = &function.kernel {
                self.write_kernel(source, function, numeric);
            }
            if self.entry_targets.contains(&function.target) {
                self.write_start(source, function);
            }
        }
    }

    fn write_execution(&self, source: &mut Code) {
        source.open("struct FunctionExecution {\n");
        source.push_str(if self.has_native_calls() {
            "active: Option<FunctionActive>,\n"
        } else {
            "active: Option<FunctionState>,\n"
        });
        for family in self.native_families() {
            let prefix = family.native_prefix();
            source.push_str(&format!("{prefix}_caller: Option<{family}Return>,\n"));
        }
        let tails = self
            .return_families()
            .iter()
            .any(|family| self.has_step(*family, StepKind::Tail));
        if tails {
            source.push_str("pending_entry: bool,\n");
        }
        for family in self.return_families() {
            source.push_str(&format!(
                "{}: Vec<{family}Return>,\n",
                family.return_stack()
            ));
        }
        source.close("}\n");
        self.write_scalar_native_state(source);
        source.open("impl FunctionExecution {\n");
        source.open("fn new(active: FunctionState) -> Self {\n");
        source.open("Self {\n");
        source.push_str(&format!("active: Some({}),\n", self.active("active")));
        for family in self.native_families() {
            let prefix = family.native_prefix();
            source.push_str(&format!("{prefix}_caller: None,\n"));
        }
        if tails {
            source.push_str("pending_entry: false,\n");
        }
        for family in self.return_families() {
            source.push_str(&format!("{}: Vec::new(),\n", family.return_stack()));
        }
        source.close("}\n");
        source.close("}\n");
        source.close("}\n");
        source.open("impl CallExecution for FunctionExecution {\n");
        source.open("fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {\n");
        source.push_str("if self.active.is_some() { return false; }\n");
        source.open("let active = match target {\n");
        for function in self
            .functions
            .iter()
            .filter(|function| self.entry_targets.contains(&function.target))
        {
            source.push_str(&format!(
                "{} => {}_state(point, values),\n",
                Rust::expression(&function.target),
                function_name(function.target)
            ));
        }
        source.push_str("_ => None,\n");
        source.close("};\n");
        source.push_str(&format!(
            "let Some(active) = active else {{ return false; }};\nself.active = Some({});\ntrue\n",
            self.active("active")
        ));
        source.close("}\n");
        source.open("fn retained_bytes(&self) -> usize {\n");
        let capacities = self
            .return_families()
            .into_iter()
            .map(|family| {
                format!(
                    "self.{}.capacity() * std::mem::size_of::<{family}Return>()",
                    family.return_stack()
                )
            })
            .collect::<Vec<_>>()
            .join(" + ");
        source.push_str(&format!("std::mem::size_of::<Self>() + {capacities}\n"));
        source.close("}\n");
        self.write_advance(source, false);
        if self.has_native_calls() {
            self.write_advance(source, true);
        }
        source.close("}\n");
        for family in self.native_families() {
            self.write_native_resume(source, family);
        }
    }

    fn write_protocol(&self, source: &mut Code) {
        source.push_str("#[allow(clippy::large_enum_variant, reason = \"Typed locals stay inline to avoid allocating at each generated step.\")]\n");
        source.open("enum FunctionStep {\n");
        source.push_str("Yield(FunctionState),\n");
        for family in self.native_families() {
            source.push_str(&format!("{family}Native {{ function: data::function::{family}FunctionId, site: data::source::HostCallSite, arguments: Box<CallValues>, caller: Option<{family}Return> }},\n"));
            source.push_str(&format!(
                "{family}NativeComplete {{ value: {} }},\n",
                family.value_type()
            ));
        }
        if self.has_canonical_step() {
            source.push_str("Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },\n");
        }
        for family in self.return_families() {
            if self.has_step(family, StepKind::Call) {
                source.push_str(&format!(
                    "{family}Call {{ callee: FunctionState, caller: {family}Return }},\n"
                ));
            }
            if self.has_step(family, StepKind::Tail) {
                source.push_str(&format!("{family}Tail {{ callee: FunctionState }},\n"));
            }
            if self.has_step(family, StepKind::Return) {
                source.push_str(&format!("{family} {{ value: {} }},\n", family.value_type()));
            }
            if self.has_native_bridge(family) {
                source.push_str(&format!("{family}ScalarBridge {{ function: data::function::{family}FunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: {family}Return }},\n"));
            }
            if self.has_step(family, StepKind::Bridge) {
                source.push_str(&format!("{family}Bridge {{ function: data::function::{family}FunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: {family}Return }},\n"));
            }
        }
        source.close("}\n");
    }

    fn has_canonical_step(&self) -> bool {
        self.functions.iter().any(|function| {
            function.kernel.is_some()
                || function
                    .shape
                    .calls
                    .iter()
                    .any(|call| matches!(call.output, CallLocal::Int(_) | CallLocal::Nullary(_)))
                || function.shape.points.iter().any(|action| match action {
                    CallPoint::Terminator(CallTerminator::Match(_))
                    | CallPoint::Scalar(CallScalar::CompoundField { .. })
                    | CallPoint::Scalar(CallScalar::Integer(..))
                    | CallPoint::Scalar(CallScalar::IntList(IntListInstruction::Index {
                        ..
                    }))
                    | CallPoint::Scalar(CallScalar::Index { .. })
                    | CallPoint::Interpreted => true,
                    CallPoint::Scalar(CallScalar::Region { outputs, .. }) => !outputs.is_empty(),
                    CallPoint::Tail(index) => {
                        let tail = &function.shape.tails[*index];
                        self.direct_callee(function.target, tail.target, &tail.args)
                            .is_none()
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
                    && function.shape.tails.iter().any(|tail| {
                        self.direct_callee(function.target, tail.target, &tail.args)
                            .is_some()
                    })
            }
            StepKind::Call | StepKind::Bridge => function.shape.calls.iter().any(|call| {
                if call_family(call) != family {
                    return false;
                }
                let direct = match call.target {
                    CallContractTarget::Static(target) => self
                        .direct_callee(function.target, target, &call.args)
                        .is_some(),
                    _ => self
                        .functions
                        .iter()
                        .any(|callee| callee.accepts_call(call)),
                };
                match kind {
                    StepKind::Call => direct,
                    _ => {
                        (!matches!(call.target, CallContractTarget::Static(_)) || !direct)
                            && Self::native_input(call).is_none()
                    }
                }
            }),
        })
    }

    fn static_callee(
        &self,
        target: CallTarget,
        args: &[CallLocal],
    ) -> Option<&CallFunction<'graph, Graph>> {
        self.functions.iter().copied().find(|callee| {
            let parameters = &callee.shape.locals[callee.shape.entry()];
            callee.target == target
                && callee.shape.parameter_count == parameters.len()
                && callee.matches_parameters(args)
        })
    }

    // Crossing into a Native loop must retain its entry's producer and binding
    // selection. Its generated fallback still connects the loop's own work.
    fn direct_callee(
        &self,
        caller: CallTarget,
        target: CallTarget,
        args: &[CallLocal],
    ) -> Option<&CallFunction<'graph, Graph>> {
        self.static_callee(target, args)
            .filter(|callee| !callee.native_loop || callee.target == caller)
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
        source.open(&format!("const RETURNS: [fn(&data::compiled::numeric::NumericValues) -> FunctionStep; {}] = [\n", function.kernel_returns.len()));
        for local in &function.kernel_returns {
            let value = local.value_expression();
            source.push_str(&format!(
                "|values| FunctionStep::{} {{ value: {value} }},\n",
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
        source.push_str(&format!("FunctionStep::Canonical {{ target: {}, point: POINTS[point], values: Box::new(CallValues {{ ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), ..CallValues::default() }}) }}\n", Rust::expression(&function.target)));
        source.close("},\n");
        source.push_str(
            "data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](values),\n",
        );
        source.close("}\n");
        source.close("}\n");
    }

    fn write_start(&self, source: &mut Code, function: &CallFunction<'_, Graph>) {
        let inputs = if function.shape.locals.iter().any(|locals| {
            locals
                .iter()
                .any(|local| !matches!(local, CallLocal::Nil(_)))
        }) {
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
                        CallContractTarget::FloatValue(local) => {
                            Some((call, "float_function_target", local.0))
                        }
                        CallContractTarget::StringValue(local) => {
                            Some((call, "string_function_target", local.0))
                        }
                        CallContractTarget::BitArrayValue(local) => {
                            Some((call, "bit_array_function_target", local.0))
                        }
                        CallContractTarget::UtfCodepointValue(local) => {
                            Some((call, "utf_codepoint_function_target", local.0))
                        }
                        CallContractTarget::NilValue(local) => {
                            Some((call, "nil_function_target", local.0))
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

    fn write_continuations(&self, source: &mut Code, family: CallFamily) {
        let canonical = self.has_canonical_step();
        let bridge = self.has_step(family, StepKind::Bridge);
        let completes = canonical || bridge || self.has_step(family, StepKind::Return);
        if !completes && self.has_step(family, StepKind::Call) {
            source.push_str("#[allow(dead_code, reason = \"Caller locals stay owned until the non-returning callee is cancelled.\")]\n");
        }
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
        if !completes {
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
            source.push_str(if family == CallFamily::Nil {
                "let () = result;\n"
            } else {
                "let _ = result;\n"
            });
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
                    | CallLocal::BoolFunction { type_, .. }
                    | CallLocal::FloatFunction { type_, .. }
                    | CallLocal::StringFunction { type_, .. }
                    | CallLocal::BitArrayFunction { type_, .. }
                    | CallLocal::UtfCodepointFunction { type_, .. }
                    | CallLocal::NilFunction { type_, .. } => {
                        format!("result.with_type({})", Rust::expression(type_))
                    }
                    CallLocal::Nullary(value) => {
                        source.push_str(&format!(
                            "let Some(result) = result.nullary(&{}) else {{\n",
                            Rust::expression(value.constructors.as_slice())
                        ));
                        source.push_str(&format!("let {} = result;\nreturn FunctionState::Canonical {{ target: {}, point: {}, values: {} }};\n}};\n", local_name(&call.output), Rust::expression(&function.target), Rust::expression(&function.shape.checkpoints[call.point + 1]), values_with_result(&function.shape.locals[call.point + 1], false, Some(&call.output))));
                        "result".to_owned()
                    }
                    _ => "result".to_owned(),
                };
                source.push_str(&format!(
                    "let {} = {value};\n{}\n",
                    if matches!(call.output, CallLocal::Nil(_)) {
                        "()".to_owned()
                    } else {
                        local_name(&call.output)
                    },
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

    fn write_return(&self, source: &mut Code, function: &CallFunction<'_, Graph>, index: usize) {
        let returning = &function.shape.returns[index];
        source.push_str(&format!(
            "FunctionStep::{} {{ value: {} }}\n",
            target_family(function.target),
            if matches!(returning.value, CallLocal::Nullary(_)) {
                format!("{}.into()", local_expression(&returning.value, false))
            } else {
                local_expression(&returning.value, false)
            }
        ));
    }

    fn write_tail(&self, source: &mut Code, function: &CallFunction<'_, Graph>, index: usize) {
        let tail = &function.shape.tails[index];
        let checkpoint = function.shape.checkpoints[tail.point];
        let locals = &function.shape.locals[tail.point];
        if self.is_native(tail.target, &tail.args) {
            source.open(&format!(
                "if ops.supports_{}_native({}) {{\n",
                target_family(tail.target).name().to_lowercase(),
                target_id(tail.target)
            ));
            source.push_str("*budget -= 1;\n");
            self.write_native_request(source, tail.target, &tail.site, &tail.args, "None");
            source.close("}\n");
        }
        if let Some(callee) = self.direct_callee(function.target, tail.target, &tail.args) {
            let parameters = &callee.shape.locals[callee.shape.entry()];
            let fields = parameters
                .iter()
                .zip(tail.args.iter())
                .map(|(parameter, argument)| field_assignment(parameter, argument))
                .collect::<Vec<_>>()
                .join(", ");
            source.push_str(&format!(
                "FunctionStep::{}Tail {{ callee: FunctionState::{} {{ {fields} }} }}\n",
                target_family(function.target),
                state_name(callee.target, callee.shape.entry())
            ));
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
        point: usize,
        terminator: &CallTerminator<'_>,
    ) {
        match terminator {
            CallTerminator::Match(matcher) => {
                self.write_compound_match(source, function, point, matcher)
            }
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
            .zip(edge.args().iter().filter_map(|local| {
                CallLocal::inspect(local).or_else(|| {
                    function
                        .shape
                        .locals
                        .iter()
                        .flatten()
                        .find(|value| value.canonical() == *local)
                        .cloned()
                })
            }))
            .map(|(parameter, argument)| field_assignment(parameter, &argument))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{}State::Point{point} {{ {fields} }}",
            function_state(function.target)
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
                if self.is_native(*target, &call.args) {
                    source.open(&format!(
                        "if ops.supports_{}_native({}) {{\n",
                        target_family(*target).name().to_lowercase(),
                        target_id(*target)
                    ));
                    self.write_native_request(
                        source,
                        *target,
                        &call.site,
                        &call.args,
                        &format!("Some({caller})"),
                    );
                    source.close("}\n");
                }
                if let Some(callee) = self.direct_callee(function.target, *target, &call.args) {
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
            CallContractTarget::FloatValue(local) => {
                source.push_str(&format!("let callable = &float_function{};\nlet captures = callable.captures();\nlet target = callable.target();\n", local.0));
                self.write_dynamic(source, call, CallFamily::Float, &caller);
            }
            CallContractTarget::StringValue(local) => {
                source.push_str(&format!("let callable = &string_function{};\nlet captures = callable.captures();\nlet target = callable.target();\n", local.0));
                self.write_dynamic(source, call, CallFamily::String, &caller);
            }
            CallContractTarget::BitArrayValue(local) => {
                source.push_str(&format!("let callable = &bit_array_function{};\nlet captures = callable.captures();\nlet target = callable.target();\n", local.0));
                self.write_dynamic(source, call, CallFamily::BitArray, &caller);
            }
            CallContractTarget::UtfCodepointValue(local) => {
                source.push_str(&format!("let callable = &utf_codepoint_function{};\nlet captures = callable.captures();\nlet target = callable.target();\n", local.0));
                self.write_dynamic(source, call, CallFamily::UtfCodepoint, &caller);
            }
            CallContractTarget::NilValue(local) => {
                source.push_str(&format!("let callable = &nil_function{};\nlet captures = callable.captures();\nlet target = callable.target();\n", local.0));
                self.write_dynamic(source, call, CallFamily::Nil, &caller);
            }
        };
        let id = match call.target {
            CallContractTarget::Static(target) => target_id(target),
            _ => "target".to_owned(),
        };
        if let Some(input) = Self::native_input(call) {
            source.push_str(&format!("FunctionStep::{family}ScalarBridge {{ function: {id}, site: {}, input: {input}, caller: {caller} }}\n", Rust::expression(&call.site)));
            return;
        }
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
                    !callee.native_loop
                        && target_family(callee.target) == entry.family
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
                                let value = capture_input_expression(local);
                                format!("{}: {value}", local_name(local))
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
    fn return_families(&self) -> Vec<CallFamily> {
        return_families()
            .into_iter()
            .filter(|family| {
                self.functions.iter().any(|function| {
                    target_family(function.target) == *family
                        || function
                            .shape
                            .calls
                            .iter()
                            .any(|call| call_family(call) == *family)
                })
            })
            .collect()
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

fn return_families() -> [CallFamily; 16] {
    [
        CallFamily::Int,
        CallFamily::Bool,
        CallFamily::IntFunction,
        CallFamily::BoolFunction,
        CallFamily::Float,
        CallFamily::String,
        CallFamily::BitArray,
        CallFamily::UtfCodepoint,
        CallFamily::Nil,
        CallFamily::FloatFunction,
        CallFamily::StringFunction,
        CallFamily::BitArrayFunction,
        CallFamily::UtfCodepointFunction,
        CallFamily::NilFunction,
        CallFamily::Custom,
        CallFamily::Tuple,
    ]
}

fn target_family(target: CallTarget) -> CallFamily {
    match target {
        CallTarget::Custom(_) => CallFamily::Custom,
        CallTarget::Tuple(_) => CallFamily::Tuple,
        CallTarget::Int(_) => CallFamily::Int,
        CallTarget::Bool(_) => CallFamily::Bool,
        CallTarget::IntFunction(_) => CallFamily::IntFunction,
        CallTarget::BoolFunction(_) => CallFamily::BoolFunction,
        CallTarget::Float(_) => CallFamily::Float,
        CallTarget::String(_) => CallFamily::String,
        CallTarget::BitArray(_) => CallFamily::BitArray,
        CallTarget::UtfCodepoint(_) => CallFamily::UtfCodepoint,
        CallTarget::Nil(_) => CallFamily::Nil,
        CallTarget::FloatFunction(_) => CallFamily::FloatFunction,
        CallTarget::StringFunction(_) => CallFamily::StringFunction,
        CallTarget::BitArrayFunction(_) => CallFamily::BitArrayFunction,
        CallTarget::UtfCodepointFunction(_) => CallFamily::UtfCodepointFunction,
        CallTarget::NilFunction(_) => CallFamily::NilFunction,
    }
}
fn target_id(target: CallTarget) -> String {
    match target {
        CallTarget::Custom(id) => Rust::expression(&id),
        CallTarget::Tuple(id) => Rust::expression(&id),
        CallTarget::Int(id) => Rust::expression(&id),
        CallTarget::Bool(id) => Rust::expression(&id),
        CallTarget::IntFunction(id) => Rust::expression(&id),
        CallTarget::BoolFunction(id) => Rust::expression(&id),
        CallTarget::Float(id) => Rust::expression(&id),
        CallTarget::String(id) => Rust::expression(&id),
        CallTarget::BitArray(id) => Rust::expression(&id),
        CallTarget::UtfCodepoint(id) => Rust::expression(&id),
        CallTarget::Nil(id) => Rust::expression(&id),
        CallTarget::FloatFunction(id) => Rust::expression(&id),
        CallTarget::StringFunction(id) => Rust::expression(&id),
        CallTarget::BitArrayFunction(id) => Rust::expression(&id),
        CallTarget::UtfCodepointFunction(id) => Rust::expression(&id),
        CallTarget::NilFunction(id) => Rust::expression(&id),
    }
}
fn function_name(target: CallTarget) -> String {
    format!(
        "calls_{}_{}",
        target_family(target).name().to_lowercase(),
        target.index()
    )
}
fn function_state(target: CallTarget) -> String {
    format!("{}{}", target_family(target), target.index())
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
        CallContractTarget::FloatValue(_) => CallFamily::Float,
        CallContractTarget::StringValue(_) => CallFamily::String,
        CallContractTarget::BitArrayValue(_) => CallFamily::BitArray,
        CallContractTarget::UtfCodepointValue(_) => CallFamily::UtfCodepoint,
        CallContractTarget::NilValue(_) => CallFamily::Nil,
    }
}

fn state(target: CallTarget, point: usize, locals: &[CallLocal]) -> String {
    format!(
        "FunctionState::{}{}",
        state_name(target, point),
        pattern(locals)
    )
}

fn canonical(target: CallTarget, point: CompiledCheckpoint, locals: &[CallLocal]) -> String {
    format!(
        "FunctionStep::Canonical {{ target: {}, point: {}, values: {} }}",
        Rust::expression(&target),
        Rust::expression(&point),
        values(locals, false)
    )
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
fn primitive_list_family(local: &CallLocal) -> Option<(&'static str, String)> {
    Some(match local {
        CallLocal::BoolList { type_id, .. } => ("bool", Rust::expression(type_id)),
        CallLocal::FloatList { type_id, .. } => ("float", Rust::expression(type_id)),
        CallLocal::StringList { type_id, .. } => ("string", Rust::expression(type_id)),
        CallLocal::BitArrayList { type_id, .. } => ("bit_array", Rust::expression(type_id)),
        CallLocal::UtfCodepointList { type_id, .. } => ("utf_codepoint", Rust::expression(type_id)),
        CallLocal::NilList { type_id, .. } => ("nil", Rust::expression(type_id)),
        _ => return None,
    })
}

fn list_family(local: &PrimitiveListLocal) -> (&'static str, String) {
    match local {
        PrimitiveListLocal::Bool { type_id, .. } => ("bool", Rust::expression(type_id)),
        PrimitiveListLocal::Float { type_id, .. } => ("float", Rust::expression(type_id)),
        PrimitiveListLocal::String { type_id, .. } => ("string", Rust::expression(type_id)),
        PrimitiveListLocal::BitArray { type_id, .. } => ("bit_array", Rust::expression(type_id)),
        PrimitiveListLocal::UtfCodepoint { type_id, .. } => {
            ("utf_codepoint", Rust::expression(type_id))
        }
        PrimitiveListLocal::Nil { type_id, .. } => ("nil", Rust::expression(type_id)),
    }
}

fn test_expression(test: &CallTest) -> String {
    match test {
        CallTest::Not(local) => format!("!bool{}", local.0),
        CallTest::Compare(comparison, left, right) => {
            super::integer_comparison(comparison, operand(*left), operand(*right))
        }
        CallTest::FloatCompare(comparison, left, right) => {
            // Explicit Float comparisons preserve NaN even for the same local.
            let method = match comparison {
                FloatComparison::Less => "lt",
                FloatComparison::LessEqual => "le",
                FloatComparison::Greater => "gt",
                FloatComparison::GreaterEqual => "ge",
            };
            format!("f64::{method}(&float{}, &float{})", left.0, right.0)
        }
        CallTest::Equal {
            left,
            right,
            negate,
        } => {
            let condition = if let Some((family, _)) = primitive_list_family(left) {
                format!(
                    "ops.primitive_lists().{family}_equal(&{}, &{})",
                    local_name(left),
                    local_name(right)
                )
            } else if let (CallLocal::Float(left), CallLocal::Float(right)) = (left, right) {
                format!("f64::eq(&float{}, &float{})", left.0, right.0)
            } else if matches!((left, right), (CallLocal::Nil(_), CallLocal::Nil(_))) {
                return (!negate).to_string();
            } else if left == right {
                format!(
                    "PartialEq::eq(&{}, &{})",
                    local_expression(left, false),
                    local_expression(right, false)
                )
            } else {
                return format!(
                    "{} {} {}",
                    local_expression(left, false),
                    if *negate { "!=" } else { "==" },
                    local_expression(right, false)
                );
            };
            if *negate {
                format!("!({condition})")
            } else {
                condition
            }
        }
        CallTest::Length {
            list,
            length,
            at_least,
        } => length_expression(&local_name(list), *length, *at_least),
        CallTest::StringPrefix { value, prefix } => format!(
            "string{}.starts_with({}.as_bytes())",
            value.0,
            Rust::expression(prefix.as_str())
        ),
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
        CallScalar::CompoundField { .. } => {}
        CallScalar::Nullary {
            output,
            constructor,
        } => source.push_str(&format!(
            "let nullary{} = CallNullary::new({});\n",
            output.0,
            Rust::expression(constructor)
        )),
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
        CallScalar::Float(output, operation) => {
            let expression = match operation {
                FloatOperation::Value(value) => Rust::expression(value),
                FloatOperation::Add(left, right) => format!("float{} + float{}", left.0, right.0),
                FloatOperation::Subtract(left, right) => {
                    format!("float{} - float{}", left.0, right.0)
                }
                FloatOperation::Multiply(left, right) => {
                    format!("float{} * float{}", left.0, right.0)
                }
                FloatOperation::Divide(left, right) => format!(
                    "if float{} == 0.0 {{ 0.0 }} else {{ float{} / float{} }}",
                    right.0, left.0, right.0
                ),
            };
            source.push_str(&format!("let float{} = {expression};\n", output.0));
        }
        CallScalar::String(output, operation) => {
            let expression = match operation {
                StringOperation::Literal(text) => {
                    format!("StringValue::from({})", Rust::expression(*text))
                }
                StringOperation::DropPrefix { value, bytes } => {
                    format!("string{}.slice({bytes}..string{}.len())", value.0, value.0)
                }
            };
            source.push_str(&format!("let string{} = {expression};\n", output.0));
        }
        CallScalar::Nil => {}
        CallScalar::List { output, operation } => {
            let (family, type_id) = list_family(output);
            let elements = |elements: &[CallLocal]| {
                if family == "nil" {
                    elements.len().to_string()
                } else if elements.len() == 1 && matches!(family, "string" | "bit_array") {
                    format!(
                        "std::slice::from_ref(&{})",
                        local_expression(&elements[0], false)
                    )
                } else {
                    format!(
                        "&[{}]",
                        elements
                            .iter()
                            .map(|local| local_expression(local, true))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            };
            let expression = match operation {
                PrimitiveListOperation::Value(values) => format!(
                    "ops.primitive_lists().{family}_value({type_id}, {})",
                    elements(values)
                ),
                PrimitiveListOperation::Spread {
                    elements: values,
                    tail,
                } => format!(
                    "ops.primitive_lists().{family}_prepend({type_id}, {}, &{})",
                    elements(values),
                    local_name(tail)
                ),
                PrimitiveListOperation::Tail { list, count } => format!(
                    "ops.primitive_lists().{family}_tail(&{}, {type_id}, {count})",
                    local_name(list)
                ),
            };
            source.push_str(&format!(
                "let {} = {expression};\n",
                local_name(&output.canonical())
            ));
        }
        // The bounds check and typed read precede the canonical Index charge.
        CallScalar::Index { .. } => {}
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
    use std::collections::BTreeSet;

    use super::super::shape::NumericComparison;
    use super::shape::{PrimitiveListLocal, PrimitiveListOperation};
    use super::{
        CallBoolean, CallCodegen, CallFamily, CallGroupCodegen, CallLocal, CallPoint, CallScalar,
        CallTarget, CallTerminator, CallTest, Code, FloatComparison, IntListInstruction,
        IntListTest, IntegerOperand, NumericInteger, NumericOperation, integer_expression,
        target_id, test_expression, write_scalar,
    };
    use crate::plan::execution::function::{
        BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
        FloatFunctionFunctionId, FloatFunctionId, IntFunctionFunctionId, IntFunctionId,
        NilFunctionFunctionId, NilFunctionId, StringFunctionFunctionId, StringFunctionId,
        UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::{
        ArithmeticNode, ArithmeticOperand, ArithmeticOutput, ArithmeticRegion, BitArrayListLocalId,
        BitArrayLocalId, BlockId, BoolLocalId, FloatListLocalId, FloatLocalId, IntListLocalId,
        IntLocalId, NilListLocalId, NilLocalId, ParamLocal, ParamSlot, StringListLocalId,
        StringLocalId, UtfCodepointLocalId, native_proof,
    };
    use crate::plan::execution::type_::{
        BitArrayListTypeId, FloatListTypeId, IntListTypeId, ListTypeId, NilListTypeId,
        StringListTypeId, ValueShapeId,
    };

    #[test]
    fn primitive_list_emission_preserves_counts_borrows_and_owned_elements() {
        let cases = [
            (
                PrimitiveListLocal::Nil {
                    local: NilListLocalId(0),
                    type_id: NilListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                PrimitiveListOperation::Value(vec![
                    CallLocal::Nil(NilLocalId(0)),
                    CallLocal::Nil(NilLocalId(1)),
                ]),
                "let nil_list0 = ops.primitive_lists().nil_value(data::type_::NilListTypeId {\n    list_type: data::type_::ListTypeId(0),\n}, 2);\n",
            ),
            (
                PrimitiveListLocal::String {
                    local: StringListLocalId(0),
                    type_id: StringListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                PrimitiveListOperation::Value(vec![CallLocal::String(StringLocalId(0))]),
                "let string_list0 = ops.primitive_lists().string_value(data::type_::StringListTypeId {\n    list_type: data::type_::ListTypeId(0),\n}, std::slice::from_ref(&string0));\n",
            ),
            (
                PrimitiveListLocal::BitArray {
                    local: BitArrayListLocalId(0),
                    type_id: BitArrayListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                PrimitiveListOperation::Value(vec![CallLocal::BitArray(BitArrayLocalId(0))]),
                "let bit_array_list0 = ops.primitive_lists().bit_array_value(data::type_::BitArrayListTypeId {\n    list_type: data::type_::ListTypeId(0),\n}, std::slice::from_ref(&bit_array0));\n",
            ),
            (
                PrimitiveListLocal::Float {
                    local: FloatListLocalId(0),
                    type_id: FloatListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                PrimitiveListOperation::Value(vec![CallLocal::Float(FloatLocalId(0))]),
                "let float_list0 = ops.primitive_lists().float_value(data::type_::FloatListTypeId {\n    list_type: data::type_::ListTypeId(0),\n}, &[float0]);\n",
            ),
            (
                PrimitiveListLocal::String {
                    local: StringListLocalId(0),
                    type_id: StringListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                PrimitiveListOperation::Spread {
                    elements: vec![
                        CallLocal::String(StringLocalId(0)),
                        CallLocal::String(StringLocalId(1)),
                    ],
                    tail: CallLocal::StringList {
                        local: StringListLocalId(1),
                        type_id: StringListTypeId {
                            list_type: ListTypeId(0),
                        },
                    },
                },
                "let string_list0 = ops.primitive_lists().string_prepend(data::type_::StringListTypeId {\n    list_type: data::type_::ListTypeId(0),\n}, &[string0.clone(), string1.clone()], &string_list1);\n",
            ),
            (
                PrimitiveListLocal::String {
                    local: StringListLocalId(0),
                    type_id: StringListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                PrimitiveListOperation::Tail {
                    list: CallLocal::StringList {
                        local: StringListLocalId(1),
                        type_id: StringListTypeId {
                            list_type: ListTypeId(0),
                        },
                    },
                    count: 2,
                },
                "let string_list0 = ops.primitive_lists().string_tail(&string_list1, data::type_::StringListTypeId {\n    list_type: data::type_::ListTypeId(0),\n}, 2);\n",
            ),
        ];
        for (output, operation, expected) in cases {
            let mut generated = Code::default();
            write_scalar(&mut generated, &CallScalar::List { output, operation });
            assert_eq!(generated.as_str(), expected);
        }
    }

    #[test]
    fn call_free_program_emits_no_group_declarations() {
        let plan = crate::ExecutionPlan::from_module_plan(
            crate::plan_module(
                crate::compile_typed_module(
                    "example",
                    "src/example.gleam",
                    "pub fn main() { Nil }",
                )
                .unwrap(),
            )
            .unwrap(),
        );
        let codegen = CallCodegen::new(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert!(codegen.is_empty());
        let mut generated = Code::default();
        generated.push_str("existing declarations\n");
        codegen.write_code(&mut generated);
        assert_eq!(generated.as_str(), "existing declarations\n");
    }

    #[test]
    fn primitive_index_keeps_a_canonical_boundary_without_integer_calls() {
        let source = r#"
fn predicate(value: Float) { value >. 0.0 }
fn identity(value: Bool) { value }
pub fn first(values: List(Float)) {
  let checked = case values { [head, ..] -> predicate(head) [] -> False }
  identity(checked)
}
pub fn main() { first([1.0]) }
"#;
        let plan = crate::ExecutionPlan::from_module_plan(
            crate::plan_module(
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap(),
            )
            .unwrap(),
        );
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let entries = program
            .functions
            .iter()
            .filter(|function| function.shape.root)
            .map(|function| function.target)
            .collect();
        let codegen =
            CallGroupCodegen::new(program.functions.iter().collect(), entries, BTreeSet::new());
        assert!(codegen.has_canonical_step());
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert!(generated.as_str().contains("FunctionStep::Canonical"));
    }

    #[test]
    fn separate_echo_blocks_preserve_each_canonical_callee_boundary() {
        let source = r#"
fn leaf(value: Bool) { !value }
fn traced(value: Bool) { let result = leaf(value) echo result echo result result }
fn counted(value: Bool) {
  let result = traced(value)
  case result { True -> 1 False -> 0 }
}
pub fn main() { counted(False) }
"#;
        let plan = crate::ExecutionPlan::from_module_plan(
            crate::plan_module(
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap(),
            )
            .unwrap(),
        );
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let traced = program
            .functions
            .iter()
            .position(|function| {
                function
                    .shape
                    .points
                    .iter()
                    .filter(|point| matches!(point, CallPoint::Interpreted))
                    .count()
                    == 2
            })
            .unwrap();
        let groups = super::group::CallGroup::inspect(&program.functions);
        let group = groups
            .iter()
            .find(|group| group.members.contains(&traced) && !group.entries.contains(&traced))
            .unwrap();
        let codegen = CallGroupCodegen::new(
            group
                .members
                .iter()
                .map(|&index| &program.functions[index])
                .collect(),
            group
                .entries
                .iter()
                .map(|&index| program.functions[index].target)
                .collect(),
            BTreeSet::new(),
        );
        let traced = &program.functions[traced];
        let points = traced
            .shape
            .points
            .iter()
            .enumerate()
            .filter_map(|(point, action)| matches!(action, CallPoint::Interpreted).then_some(point))
            .map(|point| codegen.local_point(traced, point))
            .collect::<Vec<_>>();
        assert_eq!(points, [true, true]);
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert!(generated.as_str().contains("FunctionStep::Canonical"));
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(1.into())
        );
        assert_eq!(
            echo.iter().map(|output| output.value()).collect::<Vec<_>>(),
            [&crate::Value::Bool(true), &crate::Value::Bool(true)]
        );
    }

    #[test]
    fn canonical_only_callable_locals_need_no_generated_type_import() {
        let source = r#"
fn compare(value: String) {
  let calculate = fn() { value }
  [calculate] == [calculate]
}
pub fn main() { compare("input") }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let compare = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Bool(BoolFunctionId(1)))
            .unwrap();
        let group = CallGroupCodegen::new(vec![compare], Vec::new(), BTreeSet::new());
        let mut generated = Code::default();
        group.write_code(&mut generated);
        assert_eq!(generated.as_str().matches("StringCallable").count(), 0);
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Bool(true)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn string_boolean_kernels_transfer_the_predicate_column_and_release_range_inputs() {
        let source = r#"
fn prefix(value: String, accepted: Bool) -> Bool {
  case value { "pre" <> _ -> accepted _ -> False }
}
pub fn main() { let calculate = prefix calculate("prefix", True) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let codegen = CallCodegen::new(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let function = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Bool(BoolFunctionId(1)))
            .unwrap();
        let kernel = function
            .kernel
            .as_ref()
            .expect("prefix predicate uses the String kernel");
        let group = CallGroupCodegen::new(
            codegen.functions.iter().collect(),
            Vec::new(),
            BTreeSet::new(),
        );
        let mut step = Code::default();
        group.write_kernel_step(&mut step, function, kernel, function.shape.entry());
        assert_eq!(
            step.as_str(),
            "let mut values = ops.strings(&[], &[bool0], [string0]);\nlet progress = string_bool_1(0, &mut values, budget);\ncalls_bool_1_kernel(progress, values, ops)\n"
        );
        let mut completion = Code::default();
        group.write_kernel(&mut completion, function, kernel);
        assert!(completion.as_str().contains("let value = values.bools[0];\n            values.release_inputs();\n            FunctionStep::Bool { value }\n"));
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Bool(true)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn mixed_string_and_scalar_native_groups_restore_the_caller_at_typed_delivery() {
        use crate::{
            HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
            StatelessHostProfile, StringValue,
        };
        use num_bigint::BigInt;
        let source = r#"
@external(erlang, "example", "keep")
fn keep(value: Int) -> Int
@external(erlang, "example", "append")
fn append(value: String) -> String
fn number(value: Int) { keep(value) + 1 }
fn text(value: String) { let result = append(value) result }
pub fn main() { let _ = number(7) text("input") }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(BigInt,), BigInt, _>("keep", |value| value)
            .unwrap()
            .with_function::<(StringValue,), StringValue, _>("append", |value: StringValue| {
                format!("{}!", value.as_str().unwrap()).into()
            })
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let execution = hosted.execution();
        let codegen = CallCodegen::new(
            &execution.program.functions,
            &execution.program.common.custom_types,
            &execution.program.common.value_shapes,
        );
        let group = CallGroupCodegen::new(
            codegen.functions.iter().collect(),
            Vec::new(),
            codegen.native_targets.clone(),
        );
        assert!(group.has_native());
        assert!(group.has_native_calls());
        let mut delivery = Code::default();
        group.write_native_resume(&mut delivery, CallFamily::String);
        assert_eq!(
            delivery.as_str(),
            r#"impl StringNativeExecution for FunctionExecution {
    fn resume_native(mut self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
        let active = if let Some(caller) = self.native_caller.take().or_else(|| self.string_returns.pop()) {
            caller.small(value)
        } else {
            FunctionState::StringNativeComplete { value }
        };
        self.active = Some(FunctionActive::Running(active));
        self
    }
}
"#
        );
        let mut generated = Code::default();
        group.write_code(&mut generated);
        assert!(generated.as_str().contains("Ok(Some(CallProgress::Complete { output: CallOutput::String(value), execution: self }))"));
        assert!(!generated.as_str().contains("native_result"));
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        assert_eq!(
            host.block_on(hosted.run_main(&host, &mut (), &mut echo))
                .unwrap()
                .try_into_value()
                .unwrap(),
            crate::Value::String("input!".into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_mixed_native_group_handles_every_supported_return_family_without_a_wildcard() {
        use crate::{
            HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
            StatelessHostProfile,
        };
        use num_bigint::BigInt;
        let source = r#"
@external(erlang, "native", "keep")
fn keep(value: Int) -> Int
fn integer() -> Int { 7 }
fn boolean() -> Bool { True }
fn floating() -> Float { 1.5 }
fn text() -> String { "kept" }
fn bits() -> BitArray { <<5:size(3)>> }
fn codepoint(value: UtfCodepoint) -> UtfCodepoint { value }
fn nil() -> Nil { Nil }
fn integer_function() { integer }
fn boolean_function() { boolean }
fn floating_function() { floating }
fn text_function() { text }
fn bits_function() { bits }
fn codepoint_function(value: UtfCodepoint) { fn() { codepoint(value) } }
fn nil_function() { nil }
fn connected(value: UtfCodepoint) {
  let integer_call = integer_function()
  let _ = integer_call()
  let boolean_call = boolean_function()
  let _ = boolean_call()
  let floating_call = floating_function()
  let _ = floating_call()
  let text_call = text_function()
  let _ = text_call()
  let bits_call = bits_function()
  let _ = bits_call()
  let codepoint_call = codepoint_function(value)
  let _ = codepoint_call()
  let nil_call = nil_function()
  let _ = nil_call()
  keep(7) + 35
}
pub fn main() { let assert <<value:utf8_codepoint>> = <<"λ">> connected(value) }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(BigInt,), BigInt, _>("keep", |value| value)
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let execution = hosted.execution();
        let codegen = CallCodegen::new(
            &execution.program.functions,
            &execution.program.common.custom_types,
            &execution.program.common.value_shapes,
        );
        let group = CallGroupCodegen::new(
            codegen.functions.iter().collect(),
            Vec::new(),
            BTreeSet::new(),
        );
        assert_eq!(group.return_families().len(), 14);
        assert!(group.has_native_calls());
        let mut generated = Code::default();
        group.write_advance(&mut generated, true);
        assert!(
            generated
                .as_str()
                .contains("CallProgress::InterpretedUtfCodepointFunction")
        );
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        assert_eq!(
            host.block_on(hosted.run_main(&host, &mut (), &mut echo))
                .unwrap()
                .try_into_value()
                .unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn scalar_native_bridges_seal_the_caller_and_charge_call_and_return_separately() {
        use crate::{
            HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
            StatelessHostProfile, compile_typed_host_program, plan_host_program,
        };
        use num_bigint::BigInt;
        let input = r#"
@external(erlang, "native", "keep_int")
fn stop_int(value: Int) -> Int
@external(erlang, "native", "keep_bool")
fn stop_bool(value: Bool) -> Bool
fn integer(value: Int) -> Int { stop_int(value) + 1 }
fn boolean(value: Bool) -> Bool { !stop_bool(value) }
pub fn main() { let _ = integer(7) let _ = boolean(True) Nil }
"#;
        let native = HostProviderModule::<StatelessHostProfile>::new("example", "example")
            .unwrap()
            .with_function::<(BigInt,), BigInt, _>("stop_int", |value| value)
            .unwrap()
            .with_function::<(bool,), bool, _>("stop_bool", |value| value)
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
        let plan =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let execution = plan.execution();
        let codegen = CallCodegen::new(
            &execution.program.functions,
            &execution.program.common.custom_types,
            &execution.program.common.value_shapes,
        );
        let group = CallGroupCodegen::new(
            codegen.functions.iter().collect(),
            Vec::new(),
            BTreeSet::new(),
        );
        assert!(group.has_native_bridge(CallFamily::Int));
        assert!(group.has_native_bridge(CallFamily::Bool));
        for (family, name) in [(CallFamily::Int, "Int"), (CallFamily::Bool, "Bool")] {
            let mut native = Code::default();
            group.write_native_bridge(&mut native, family, true, true);
            assert_eq!(
                native.as_str(),
                r#"if let CallNativeOps::FAMILY { function: target, native } = native && *target == function {
    if *budget == 0 {
        self.active = Some(FunctionActive::FAMILYCall { function, site, input, caller });
        return Ok(Some(CallProgress::Yield(self)));
    }
    *budget -= 1;
    let Some(returned) = native.call(input, site)? else { return Ok(None); };
    if *budget == 0 {
        self.active = Some(FunctionActive::FAMILYReturn { caller, returned });
        return Ok(Some(CallProgress::Yield(self)));
    }
    *budget -= 1;
    caller.resume(returned.into_value())
} else {
    return Ok(Some(CallProgress::FAMILYScalar {
        function, site, input,
        resume: Box::new(move |value| {
            self.active = Some(FunctionActive::Running(caller.resume(value)));
            self
        }),
    }));
}
"#.replace("FAMILY", name)
            );
            let mut ordinary = Code::default();
            group.write_native_bridge(&mut ordinary, family, false, true);
            assert_eq!(
                ordinary.as_str(),
                r#"return CallProgress::FAMILYScalar {
    function, site, input,
    resume: Box::new(move |value| {
        self.active = Some(FunctionActive::Running(caller.resume(value)));
        self
    }),
};
"#
                .replace("FAMILY", name)
            );
            let mut terminal = Code::default();
            group.write_native_bridge(&mut terminal, family, false, false);
            assert_eq!(
                terminal.as_str(),
                r#"CallProgress::FAMILYScalar {
    function, site, input,
    resume: Box::new(move |value| {
        self.active = Some(FunctionActive::Running(caller.resume(value)));
        self
    }),
}
"#
                .replace("FAMILY", name)
            );
        }
        let mut ordinary = Code::default();
        group.write_advance(&mut ordinary, false);
        assert!(!ordinary.as_str().contains("CallNativeOps"));
        assert!(!ordinary.as_str().contains("native.call("));
        let mut native = Code::default();
        group.write_advance(&mut native, true);
        assert!(!native.as_str().contains(".pop().expect("));
        assert!(!native.as_str().contains("resume_native_"));
    }

    #[test]
    fn call_target_ids_keep_the_exact_scalar_and_function_return_families() {
        use crate::plan::execution::function::{CustomFunctionId, TupleFunctionId};
        use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
        for (target, expected) in [
            (
                CallTarget::Custom(CustomFunctionId::new(
                    2,
                    CustomValueShape {
                        type_id: CustomTypeId(3),
                        shape_id: CustomValueShapeId(4),
                    },
                )),
                "data::function::CustomFunctionId {\n    index: 2,\n    return_shape: data::type_::CustomValueShape {\n        type_id: data::type_::CustomTypeId(3),\n        shape_id: data::type_::CustomValueShapeId(4),\n    },\n}",
            ),
            (
                CallTarget::Tuple(TupleFunctionId(2)),
                "data::function::TupleFunctionId(2)",
            ),
            (
                CallTarget::Int(IntFunctionId(2)),
                "data::function::IntFunctionId(2)",
            ),
            (
                CallTarget::Bool(BoolFunctionId(2)),
                "data::function::BoolFunctionId(2)",
            ),
            (
                CallTarget::Float(FloatFunctionId(2)),
                "data::function::FloatFunctionId(2)",
            ),
            (
                CallTarget::String(StringFunctionId(2)),
                "data::function::StringFunctionId(2)",
            ),
            (
                CallTarget::BitArray(BitArrayFunctionId(2)),
                "data::function::BitArrayFunctionId(2)",
            ),
            (
                CallTarget::UtfCodepoint(UtfCodepointFunctionId(2)),
                "data::function::UtfCodepointFunctionId(2)",
            ),
            (
                CallTarget::Nil(NilFunctionId(2)),
                "data::function::NilFunctionId(2)",
            ),
            (
                CallTarget::IntFunction(IntFunctionFunctionId(2)),
                "data::function::IntFunctionFunctionId(2)",
            ),
            (
                CallTarget::BoolFunction(BoolFunctionFunctionId(2)),
                "data::function::BoolFunctionFunctionId(2)",
            ),
            (
                CallTarget::FloatFunction(FloatFunctionFunctionId(2)),
                "data::function::FloatFunctionFunctionId(2)",
            ),
            (
                CallTarget::StringFunction(StringFunctionFunctionId(2)),
                "data::function::StringFunctionFunctionId(2)",
            ),
            (
                CallTarget::BitArrayFunction(BitArrayFunctionFunctionId(2)),
                "data::function::BitArrayFunctionFunctionId(2)",
            ),
            (
                CallTarget::UtfCodepointFunction(UtfCodepointFunctionFunctionId(2)),
                "data::function::UtfCodepointFunctionFunctionId(2)",
            ),
            (
                CallTarget::NilFunction(NilFunctionFunctionId(2)),
                "data::function::NilFunctionFunctionId(2)",
            ),
        ] {
            assert_eq!(target_id(target), expected);
        }
    }

    #[test]
    fn a_tail_only_family_needs_no_unreachable_completion_implementation() {
        let source = r#"
fn keep(value: String) -> String { value }
fn spin(flag: Bool) -> Bool { spin(flag) }
pub fn main() {
  let value = keep("value")
  let _ = spin(True)
  value
}
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            crate::HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let mut generated = Code::default();
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        codegen.write_code(&mut generated);
        assert!(generated.as_str().contains("enum BoolReturn {"));
        assert!(!generated.as_str().contains("impl BoolReturn {"));
        assert!(
            !generated
                .as_str()
                .contains("FunctionStep::Bool { value } =>")
        );
        assert!(generated.as_str().contains("impl StringReturn {"));
        let mut continuation = Code::default();
        codegen.write_continuations(&mut continuation, CallFamily::Bool);
        assert_eq!(
            continuation.as_str(),
            r#"#[allow(dead_code, reason = "Caller locals stay owned until the non-returning callee is cancelled.")]
enum BoolReturn {
    String0Call3 { string0: StringValue, string1: StringValue, bool0: bool },
}
"#
        );
        // Only preparation is tested: the valid source deliberately has no
        // normal Boolean completion and must never be executed here.
    }

    #[test]
    fn length_only_callee_and_nil_caller_keep_their_distinct_list_operations() {
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        let adjust = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(2)))
            .unwrap();
        assert!(adjust.kernel.is_some());
        let destinations = adjust
            .shape
            .points
            .iter()
            .filter_map(|point| match point {
                CallPoint::Terminator(terminator) => Some(terminator),
                _ => None,
            })
            .map(|terminator| {
                adjust
                    .shape
                    .starts
                    .iter()
                    .filter_map(|(block, point)| {
                        terminator.enters(BlockId(*block)).then_some(*point)
                    })
                    .collect::<BTreeSet<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            destinations,
            [
                BTreeSet::from([1, 12]),
                BTreeSet::from([3]),
                BTreeSet::from([4, 6, 8]),
                BTreeSet::from([9, 11]),
                BTreeSet::from([3]),
            ]
        );
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
                codegen.write_terminator(&mut source, adjust, 0, terminator);
                source.as_str().to_owned()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            terminators,
            [
                "if bool0 { Int2State::Point1 { int0 } } else { Int2State::Point12 { int0 } }\n",
                "Int2State::Point3 { int0: int1 }\n",
                "match int0 {\n    0_i128 => Int2State::Point4 {  },\n    1_i128 => Int2State::Point6 {  },\n    _ => Int2State::Point8 { int0 },\n}\n",
                "if int0 < 0_i128 { Int2State::Point9 {  } } else { Int2State::Point11 { int0 } }\n",
                "Int2State::Point3 { int0: int1 }\n",
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
        assert!(
            !generated
                .as_str()
                .contains("fn values(self) -> Box<CallValues>")
        );
        assert!(
            generated
                .as_str()
                .contains("CallProgress::Interpreted { target, point, values }")
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
pub fn main() -> Int {
  let stop = fn(value) { fail(value) }
  let _ = stop(7)
  panic as "caller must not resume"
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated.as_str().lines().next().unwrap(),
            "use data::compiled::calls::{CallArguments, CallExecution, CallInputs, CallInteger, CallOps, CallProgress, CallStorage, CallValues, IntCallable, StringValue};"
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
    fn static_identity_calls_preserve_the_existing_numeric_callee_without_capture_operations() {
        let input = r#"
fn identity(value: Int) { value }
fn forward(value: Int) { identity(value) + 1 }
pub fn main() { forward(7) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated
                .as_str()
                .lines()
                .find(|line| line.starts_with("fn function_step("))
                .unwrap(),
            "fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {"
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert!(
            !generated
                .as_str()
                .contains("fn values(self) -> Box<CallValues>")
        );
        assert!(
            generated
                .as_str()
                .contains("FunctionStep::BoolFunctionTail { callee } =>")
        );
        assert!(
            generated
                .as_str()
                .contains("FunctionStep::BoolFunction { value } =>")
        );
        assert!(
            generated
                .as_str()
                .contains("output: CallOutput::BoolFunction(value)")
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Bool(false)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_boolean_tail_cycle_emits_only_its_owned_transfer() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "fn first(flag: Bool) -> Bool { second(!flag) } fn second(flag: Bool) -> Bool { first(flag) } fn make() { first } pub fn main() { let calculate = make() calculate(True) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let cycle = program
            .functions
            .iter()
            .filter(|function| {
                matches!(function.target, CallTarget::Bool(_)) && !function.shape.tails.is_empty()
            })
            .collect::<Vec<_>>();
        assert_eq!(cycle.len(), 2);
        let targets = cycle.iter().map(|function| function.target).collect();
        let group = CallGroupCodegen::new(cycle, targets, BTreeSet::new());
        let mut code = Code::default();
        group.write_code(&mut code);
        assert!(code.as_str().contains("FunctionStep::BoolTail"));
        assert!(!code.as_str().contains("impl BoolReturn {"));
        assert!(
            !code
                .as_str()
                .contains("_ => return CallProgress::Interpreted")
        );
    }

    #[test]
    fn all_scalar_and_callable_results_share_one_exhaustive_return_dispatch() {
        let source = r#"
pub type Marker { Found }
fn custom(value: Marker) { value }
fn tuple(value: #(Int, Bool)) { value }
fn integer(value: Int) { value }
fn boolean(value: Bool) { value }
fn floating(value: Float) { value }
fn text(value: String) { value }
fn bits(value: BitArray) { value }
fn point(value: UtfCodepoint) { value }
fn nil(value: Nil) { value }
fn make_integer() { integer }
fn make_boolean() { boolean }
fn make_floating() { floating }
fn make_text() { text }
fn make_bits() { bits }
fn make_point() { point }
fn make_nil() { nil }
pub fn main() {
  let integer = make_integer()
  let boolean = make_boolean()
  let floating = make_floating()
  let text = make_text()
  let bits = make_bits()
  let point = make_point()
  let nil = make_nil()
  let assert <<codepoint:utf8_codepoint>> = <<"a">>
  let _ = integer(42)
  let _ = boolean(True)
  let _ = floating(1.5)
  let _ = text("kept")
  let _ = bits(<<7:8>>)
  let _ = point(codepoint)
  let _ = custom(Found)
  let _ = tuple(#(42, True))
  nil(Nil)
}
"#;
        use crate::{HostProviderSet, StatelessHostProfile};
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let group = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        assert_eq!(group.return_families().len(), 16);
        let mut code = Code::default();
        group.write_code(&mut code);
        for family in [
            "Int",
            "Bool",
            "Float",
            "String",
            "BitArray",
            "UtfCodepoint",
            "Nil",
        ] {
            assert!(
                code.as_str()
                    .contains(&format!("CallProgress::Interpreted{family} {{")),
                "{family}"
            );
            assert!(
                code.as_str()
                    .contains(&format!("CallProgress::Interpreted{family}Function {{")),
                "{family}"
            );
        }
        assert!(code.as_str().contains("CallProgress::InterpretedCustom {"));
        assert!(code.as_str().contains("CallProgress::InterpretedTuple {"));
        assert!(
            !code
                .as_str()
                .contains("_ => return CallProgress::Interpreted { target, point, values }")
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Nil
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
    fn float_comparisons_emit_ieee_operations_including_the_same_local() {
        let left = FloatLocalId(2);
        for right in [left, FloatLocalId(3)] {
            for (comparison, method) in [
                (FloatComparison::Less, "lt"),
                (FloatComparison::LessEqual, "le"),
                (FloatComparison::Greater, "gt"),
                (FloatComparison::GreaterEqual, "ge"),
            ] {
                assert_eq!(
                    test_expression(&CallTest::FloatCompare(comparison, left, right)),
                    format!("f64::{method}(&float2, &float{})", right.0)
                );
            }
            for (negate, expected) in [
                (false, format!("f64::eq(&float2, &float{})", right.0)),
                (true, format!("!(f64::eq(&float2, &float{}))", right.0)),
            ] {
                assert_eq!(
                    test_expression(&CallTest::Equal {
                        left: CallLocal::Float(left),
                        right: CallLocal::Float(right),
                        negate,
                    }),
                    expected
                );
            }
        }
    }

    #[test]
    fn primitive_list_equality_uses_the_typed_list_owner() {
        use crate::plan::execution::graph::FloatListLocalId;
        use crate::plan::execution::type_::{FloatListTypeId, ListTypeId};
        let type_id = FloatListTypeId {
            list_type: ListTypeId(0),
        };
        for (negate, expected) in [
            (
                false,
                "ops.primitive_lists().float_equal(&float_list2, &float_list3)",
            ),
            (
                true,
                "!(ops.primitive_lists().float_equal(&float_list2, &float_list3))",
            ),
        ] {
            assert_eq!(
                test_expression(&CallTest::Equal {
                    left: CallLocal::FloatList {
                        local: FloatListLocalId(2),
                        type_id
                    },
                    right: CallLocal::FloatList {
                        local: FloatListLocalId(3),
                        type_id
                    },
                    negate,
                }),
                expected
            );
        }
    }

    #[test]
    fn reflexive_scalar_comparisons_emit_borrowed_equality_and_nil_identity() {
        for (local, expression) in [
            (
                CallLocal::Bool(BoolLocalId(2)),
                "PartialEq::eq(&bool2, &bool2)",
            ),
            (
                CallLocal::String(StringLocalId(2)),
                "PartialEq::eq(&string2, &string2)",
            ),
            (
                CallLocal::BitArray(BitArrayLocalId(2)),
                "PartialEq::eq(&bit_array2, &bit_array2)",
            ),
            (
                CallLocal::UtfCodepoint(UtfCodepointLocalId(2)),
                "PartialEq::eq(&utf_codepoint2, &utf_codepoint2)",
            ),
        ] {
            for (negate, expected) in [
                (false, expression.to_owned()),
                (true, format!("!({expression})")),
            ] {
                assert_eq!(
                    test_expression(&CallTest::Equal {
                        left: local.clone(),
                        right: local.clone(),
                        negate,
                    }),
                    expected
                );
            }
        }
        for right in [NilLocalId(2), NilLocalId(3)] {
            for (negate, expected) in [(false, "true"), (true, "false")] {
                assert_eq!(
                    test_expression(&CallTest::Equal {
                        left: CallLocal::Nil(NilLocalId(2)),
                        right: CallLocal::Nil(right),
                        negate,
                    }),
                    expected
                );
            }
        }
    }

    #[test]
    fn distinct_scalar_comparisons_emit_the_requested_operator() {
        for (left, right, name) in [
            (
                CallLocal::Bool(BoolLocalId(2)),
                CallLocal::Bool(BoolLocalId(3)),
                "bool",
            ),
            (
                CallLocal::String(StringLocalId(2)),
                CallLocal::String(StringLocalId(3)),
                "string",
            ),
            (
                CallLocal::BitArray(BitArrayLocalId(2)),
                CallLocal::BitArray(BitArrayLocalId(3)),
                "bit_array",
            ),
            (
                CallLocal::UtfCodepoint(UtfCodepointLocalId(2)),
                CallLocal::UtfCodepoint(UtfCodepointLocalId(3)),
                "utf_codepoint",
            ),
        ] {
            for (negate, operator) in [(false, "=="), (true, "!=")] {
                assert_eq!(
                    test_expression(&CallTest::Equal {
                        left: left.clone(),
                        right: right.clone(),
                        negate,
                    }),
                    format!("{name}2 {operator} {name}3")
                );
            }
        }
    }

    #[test]
    fn nil_argument_equality_is_selected_after_a_source_call() {
        let input = r#"
fn keep(value: Nil) -> Nil { value }
fn equals(value: Nil) -> Bool { keep(value) == Nil }
fn differs(value: Nil) -> Bool { keep(value) != Nil }
pub fn main() { equals(Nil) && !differs(Nil) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let mut generated = Code::default();
        codegen.write_code(&mut generated);
        assert_eq!(
            generated
                .as_str()
                .lines()
                .map(str::trim)
                .filter(|line| matches!(*line, "let bool0 = true;" | "let bool0 = false;"))
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["let bool0 = true;", "let bool0 = false;"])
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
            (CallFamily::Float, "Float"),
            (CallFamily::String, "String"),
            (CallFamily::BitArray, "BitArray"),
            (CallFamily::UtfCodepoint, "UtfCodepoint"),
            (CallFamily::Nil, "Nil"),
            (CallFamily::FloatFunction, "FloatFunction"),
            (CallFamily::StringFunction, "StringFunction"),
            (CallFamily::BitArrayFunction, "BitArrayFunction"),
            (CallFamily::UtfCodepointFunction, "UtfCodepointFunction"),
            (CallFamily::NilFunction, "NilFunction"),
        ] {
            assert_eq!(family.to_string(), expected);
        }
    }

    #[test]
    fn boolean_and_nil_protocol_keeps_yields_and_direct_returns() {
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let mut protocol = Code::default();
        codegen.write_protocol(&mut protocol);
        assert_eq!(
            protocol.as_str(),
            r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Yield(FunctionState),
    BoolCall { callee: FunctionState, caller: BoolReturn },
    Bool { value: bool },
    Nil { value: () },
}
"#
        );
        let flip = &codegen.functions[0];
        let mut steps = Code::default();
        codegen.write_function(&mut steps, flip);
        assert_eq!(
            steps.as_str(),
            r#"FunctionState::Bool0Point0 { bool0 } => calls_bool_0_run(Bool0State::Point0 { bool0 }, ops, budget),
FunctionState::Bool0Point1 { bool0, bool1 } => calls_bool_0_run(Bool0State::Point1 { bool0, bool1 }, ops, budget),
FunctionState::Bool0Point2 { bool0, bool1, bool2 } => calls_bool_0_run(Bool0State::Point2 { bool0, bool1, bool2 }, ops, budget),
"#
        );
        let mut execution = Code::default();
        codegen.write_advance(&mut execution, false);
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
            FunctionStep::Bool { value } => {
                if let Some(caller) = self.boolean_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.boolean_returns.clear();
                    self.nil_returns.clear();
                    return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                }
            },
            FunctionStep::Nil { value } => {
                if let Some(caller) = self.nil_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.boolean_returns.clear();
                    self.nil_returns.clear();
                    return CallProgress::Complete { output: CallOutput::Nil(value), execution: self };
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
    Nil0Call1 { bool0: bool },
}
impl BoolReturn {
    fn small(self, result: bool) -> FunctionState {
        match self {
            Self::Bool0Call0 { bool0 } => {
                let bool1 = result;
                FunctionState::Bool0Point1 { bool0, bool1 }
            },
            Self::Nil0Call1 { bool0 } => {
                let bool1 = result;
                FunctionState::Nil0Point2 { bool0, bool1 }
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
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
    Yield(FunctionState),
    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
    BoolCall { callee: FunctionState, caller: BoolReturn },
    Bool { value: bool },
    Nil { value: () },
}
"#
        );
        let mut steps = Code::default();
        codegen.write_body(&mut steps, codegen.functions[0]);
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        assert_eq!(
            codegen
                .functions
                .iter()
                .map(|function| function.kernel.is_some())
                .collect::<Vec<_>>(),
            [false, false, false]
        );
        assert_eq!(
            codegen
                .functions
                .iter()
                .flat_map(|function| &function.shape.calls)
                .map(|call| matches!(call.output, CallLocal::Int(_)))
                .collect::<Vec<_>>(),
            [false, false]
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
    Yield(FunctionState),
    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
    BoolCall { callee: FunctionState, caller: BoolReturn },
    Bool { value: bool },
    Nil { value: () },
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
            let program = super::shape::CallProgram::inspect(
                &plan.program.functions,
                &plan.program.common.custom_types,
                &plan.program.common.value_shapes,
            );
            let codegen = CallGroupCodegen::new(
                program.functions.iter().collect(),
                program
                    .functions
                    .iter()
                    .map(|function| function.target)
                    .collect(),
                BTreeSet::new(),
            );
            assert!(codegen.entries.is_empty());
            assert_eq!(
                codegen
                    .functions
                    .iter()
                    .map(|function| (function.kernel.is_some(), function.shape.creations.len()))
                    .collect::<Vec<_>>(),
                [(false, 0), (false, 0), (false, 0)]
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let mut source = Code::default();
        codegen.write_protocol(&mut source);
        assert_eq!(
            source.as_str(),
            r#"#[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
enum FunctionStep {
    Yield(FunctionState),
    Int { value: i128 },
    Bool { value: bool },
    IntFunction { value: IntCallable },
    BoolFunction { value: BoolCallable },
}
"#
        );
        let mut advance = Code::default();
        codegen.write_advance(&mut advance, false);
        assert_eq!(
            advance.as_str(),
            r#"fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
    let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
    loop {
        match function_step(active, ops, budget) {
            FunctionStep::Yield(active) => {
                self.active = Some(active);
                return CallProgress::Yield(self);
            },
            FunctionStep::Int { value } => {
                if let Some(caller) = self.integer_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self };
                }
            },
            FunctionStep::Bool { value } => {
                if let Some(caller) = self.boolean_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                }
            },
            FunctionStep::IntFunction { value } => {
                if let Some(caller) = self.integer_function_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { output: CallOutput::IntFunction(value), execution: self };
                }
            },
            FunctionStep::BoolFunction { value } => {
                if let Some(caller) = self.boolean_function_returns.pop() {
                    active = caller.small(value);
                } else {
                    self.integer_returns.clear();
                    self.boolean_returns.clear();
                    self.integer_function_returns.clear();
                    self.boolean_function_returns.clear();
                    return CallProgress::Complete { output: CallOutput::BoolFunction(value), execution: self };
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
        let identity = codegen
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(1)))
            .unwrap();
        assert!(identity.kernel.is_none());
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
            r#"FunctionState::Int1Point0 { int0 } => calls_int_1_run(Int1State::Point0 { int0 }, ops, budget),
"#
        );
    }

    #[test]
    fn zero_argument_forwarding_has_no_unused_state_input() {
        let input = r#"
fn identity(value: Int) -> Int { value }
fn result() -> Int { 7 }
fn forward() -> Int { result() }
pub fn main() -> Int { let calculate = identity let _ = calculate(7) forward() }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", input).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
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
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(7.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn repeated_callable_invocations_share_one_exact_target_and_capture_entry() {
        let source = "fn identity(value: Int) { value } pub fn main() { let calculate = identity calculate(7) + calculate(8) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
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
        let program = super::shape::CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let codegen = CallGroupCodegen::new(
            program.functions.iter().collect(),
            program
                .functions
                .iter()
                .map(|function| function.target)
                .collect(),
            BTreeSet::new(),
        );
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
