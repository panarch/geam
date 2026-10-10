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
use std::collections::{BTreeMap, BTreeSet};

const BODY_CHECKPOINT_LIMIT: usize = 32;

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
                "FunctionState::{}{} => {}_run({}, ops, budget),\n",
                state_name(function.target, point),
                pattern(locals),
                function_name(function.target),
                self.body_state(function, point, &pattern(locals))
            ));
        }
    }

    pub(super) fn write_body(&self, source: &mut Code, function: &CallFunction<'_, Graph>) {
        if let Some(layout) = self.body_segments.get(&function.target.key()) {
            self.write_segmented_body(source, function, layout);
            return;
        }
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
        let points = function
            .shape
            .locals
            .iter()
            .enumerate()
            .filter(|(point, _)| self.local_point(function, *point))
            .map(|(point, _)| point)
            .collect::<Vec<_>>();
        let ops = if self.uses_ops(function, &points) {
            "ops"
        } else {
            "_ops"
        };
        source.open(&format!("fn {}_run({mutable}active: {owner}State, {ops}: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {{\n", function_name(function.target)));
        if repeats {
            source.open("loop {\n");
        }
        self.write_body_match(source, function, &points, &owner, repeats, &starts);
        if repeats {
            source.close("}\n");
        }
        source.close("}\n");
    }

    fn write_body_match(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        points: &[usize],
        owner: &str,
        repeats: bool,
        starts: &BTreeSet<usize>,
    ) {
        let layout = self.body_segments.get(&function.target.key());
        source.open("match active {\n");
        for &entry in points {
            let locals = &function.shape.locals[entry];
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
                if layout.is_some_and(|layout| {
                    layout
                        .get(&next)
                        .is_some_and(|segment| *segment != layout[&entry])
                }) {
                    source.push_str(&format!(
                        "{}ControlFlow::Continue({}){}\n",
                        if repeats { "return " } else { "" },
                        self.body_state(function, next, &pattern(&function.shape.locals[next])),
                        if repeats { ";" } else { "" }
                    ));
                    break;
                }
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
    }

    // Layout is preparation-local. Each helper accepts only checkpoints in its
    // segment, while the driver transfers ownership without growing the stack.
    pub(super) fn body_layout(
        &self,
        function: &CallFunction<'_, Graph>,
    ) -> Option<BTreeMap<usize, usize>> {
        if function.kernel.is_some() {
            return None;
        }
        let points = (0..function.shape.points.len())
            .filter(|&point| self.local_point(function, point))
            .collect::<Vec<_>>();
        if points.len() <= BODY_CHECKPOINT_LIMIT {
            return None;
        }
        let starts = segment_starts(function);
        let mut segment = function.shape.entry();
        let mut count = 0;
        Some(
            points
                .into_iter()
                .map(|point| {
                    if count == BODY_CHECKPOINT_LIMIT || starts.contains(&point) {
                        segment = point;
                        count = 0;
                    }
                    count += 1;
                    (point, segment)
                })
                .collect(),
        )
    }

    pub(super) fn body_state(
        &self,
        function: &CallFunction<'_, Graph>,
        point: usize,
        fields: &str,
    ) -> String {
        let owner = function_state(function.target);
        match self.body_segments.get(&function.target.key()) {
            Some(layout) => {
                let segment = layout[&point];
                format!(
                    "{owner}State::Segment{segment}({owner}Segment{segment}State::Point{point}{fields})"
                )
            }
            None => format!("{owner}State::Point{point}{fields}"),
        }
    }

    pub(super) fn body_result(&self, function: &CallFunction<'_, Graph>, result: String) -> String {
        if self.body_segments.contains_key(&function.target.key()) {
            format!("ControlFlow::Break({result})")
        } else {
            result
        }
    }

    fn write_segmented_body(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        layout: &BTreeMap<usize, usize>,
    ) {
        let owner = function_state(function.target);
        let name = function_name(function.target);
        let mut segments = BTreeMap::<usize, Vec<usize>>::new();
        for (&point, &segment) in layout {
            segments.entry(segment).or_default().push(point);
        }
        source.open(&format!("enum {owner}State {{\n"));
        for segment in segments.keys() {
            source.push_str(&format!(
                "Segment{segment}({owner}Segment{segment}State),\n"
            ));
        }
        source.close("}\n");
        for (segment, points) in &segments {
            source.open(&format!("enum {owner}Segment{segment}State {{\n"));
            for &point in points {
                source.push_str(&format!(
                    "Point{point}{},\n",
                    super::fields(&function.shape.locals[point])
                ));
            }
            source.close("}\n");
        }
        source.open(&format!("fn {name}_run(mut active: {owner}State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {{\n"));
        source.open("loop {\n");
        source.open("let progress = match active {\n");
        for segment in segments.keys() {
            source.push_str(&format!("{owner}State::Segment{segment}(active) => {name}_segment_{segment}(active, ops, budget),\n"));
        }
        source.close("};\n");
        source.open("match progress {\n");
        source.push_str("ControlFlow::Continue(next) => active = next,\nControlFlow::Break(step) => return step,\n");
        source.close("}\n");
        source.close("}\n");
        source.close("}\n");
        let starts = segment_starts(function);
        for (segment, points) in segments {
            let ops = if self.uses_ops(function, &points) {
                "ops"
            } else {
                "_ops"
            };
            let repeats = points.iter().any(|&point| {
                !starts.contains(&point)
                    && function.shape.points[point].is_inline()
                    && layout.get(&(point + 1)) == Some(&segment)
            });
            let mutable = if repeats { "mut " } else { "" };
            let budget = if points
                .iter()
                .any(|&point| !matches!(function.shape.points[point], CallPoint::Interpreted))
            {
                "budget"
            } else {
                "_budget"
            };
            source.open(&format!("fn {name}_segment_{segment}({mutable}active: {owner}Segment{segment}State, {ops}: &mut CallOps<'_>, {budget}: &mut usize) -> ControlFlow<FunctionStep, {owner}State> {{\n"));
            if repeats {
                source.open("loop {\n");
            }
            self.write_body_match(
                source,
                function,
                &points,
                &format!("{owner}Segment{segment}"),
                repeats,
                &starts,
            );
            if repeats {
                source.close("}\n");
            }
            source.close("}\n");
        }
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
            // A completed call publishes its restored caller before the
            // canonical handoff. Typed edges enter that handoff locally.
            || point.checked_sub(1).is_some_and(|previous| {
                matches!(function.shape.points[previous], CallPoint::Call(_))
            })
    }

    pub(super) fn local_point(&self, function: &CallFunction<'_, Graph>, point: usize) -> bool {
        self.global_point(function, point)
            // Only a generated edge needs a local interpreted handoff. Blocks
            // reached after a canonical handoff remain canonical-owned.
            || (function.kernel.is_none()
                && ((function.shape.starts.values().any(|&start| start == point)
                    && function.shape.points.iter().any(|action| {
                        matches!(action, CallPoint::Terminator(terminator)
                            if terminator.enters(function.shape.checkpoints[point].block))
                    })) || point.checked_sub(1).is_some_and(|previous| {
                    function.shape.points[previous].is_inline()
                        && !segment_starts(function).contains(&previous)
                })))
    }

    fn uses_ops(&self, function: &CallFunction<'_, Graph>, points: &[usize]) -> bool {
        function.kernel.is_some()
            || points
                .iter()
                .any(|&point| match &function.shape.points[point] {
                    CallPoint::Call(index) => {
                        let call = &function.shape.calls[*index];
                        match call.target {
                            super::CallContractTarget::Static(target) => {
                                self.is_native(target, &call.args)
                            }
                            _ => self
                                .entries
                                .iter()
                                .any(|entry| entry.matches(call_family(call), &call.args)),
                        }
                    }
                    CallPoint::Tail(index) => {
                        let tail = &function.shape.tails[*index];
                        self.is_native(tail.target, &tail.args)
                    }
                    CallPoint::Create(_) => true,
                    action => matches!(
                        action,
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
                    ),
                })
    }

    fn write_point(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        point: usize,
        repeats: bool,
    ) {
        let segmented = self.body_segments.contains_key(&function.target.key());
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
                "if *budget == 0 {{ return {}; }}\n",
                self.body_result(
                    function,
                    format!(
                        "FunctionStep::Yield({})",
                        state(function.target, point, locals)
                    )
                )
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
                    self.body_result(function, canonical(function.target, checkpoint, locals))
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
                    self.body_result(function, canonical(function.target, checkpoint, locals))
                ));
                source.close("};\n");
            }
            if let CallPoint::Scalar(CallScalar::CompoundField {
                output,
                source: value,
                index,
                custom,
                read,
            }) = action
            {
                source.open(&format!(
                    "let {} = match (|| {{\nlet field = {}.field({index})?;\n",
                    if matches!(output, CallLocal::Nil(_)) {
                        "()".to_owned()
                    } else {
                        local_name(output)
                    },
                    local_name(value)
                ));
                if *custom {
                    source.push_str(&format!(
                        "if !field.matches_type(&{}) {{ return None; }}\n",
                        Rust::expression(&read.value_type())
                    ));
                }
                source.push_str(&format!("{}\n", read.expression("field")));
                source.alternative("})() {\n");
                source.push_str(&format!(
                    "Some(value) => value,\nNone => return {},\n",
                    self.body_result(function, canonical(function.target, checkpoint, locals))
                ));
                source.close("};\n");
            }
            if !matches!(
                action,
                CallPoint::Tail(_) | CallPoint::Terminator(CallTerminator::Match(_))
            ) {
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
                        self.body_result(
                            function,
                            canonical(
                                function.target,
                                function.shape.checkpoints[point + 1],
                                &function.shape.locals[point + 1]
                            )
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
                source.open(if segmented {
                    if repeats {
                        "return ControlFlow::Continue({\n"
                    } else {
                        "ControlFlow::Continue({\n"
                    }
                } else {
                    "active = {\n"
                });
                self.write_terminator(source, function, point, terminator);
                source.close(if segmented {
                    if repeats { "});\n" } else { "})\n" }
                } else {
                    "};\ncontinue;\n"
                });
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
                "{}{}{}\n",
                prefix,
                self.body_result(function, canonical(function.target, checkpoint, locals)),
                suffix
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
    use super::{CallGroupCodegen, CallPoint, segment_starts};
    use crate::plan::execution::prepared::codegen::Code;
    use crate::plan::execution::prepared::codegen::calls::shape::CallProgram;
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn long_float_body_splits_at_an_existing_checkpoint() {
        let source = format!(
            "pub fn increment(value: Float) {{ {} value }} pub fn main() {{ let result = increment(0.0) result +. 1.0 }}",
            "let value = value +. 1.0\n".repeat(33)
        );
        let typed = crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let group = CallGroupCodegen::new(
            program.functions.iter().collect(),
            Vec::new(),
            BTreeSet::new(),
        );
        let function = program
            .functions
            .iter()
            .find(|function| function.shape.points.len() > 32)
            .unwrap();
        assert_eq!(
            group.body_segments[&function.target.key()],
            (0..32)
                .map(|point| (point, 0))
                .chain((32..64).map(|point| (point, 32)))
                .chain((64..67).map(|point| (point, 64)))
                .collect::<BTreeMap<_, _>>()
        );
    }

    #[test]
    fn segmented_body_keeps_owned_call_return_and_budget_checkpoints() {
        let source = "fn identity(value: Bool) { value } pub fn flip(value: Bool) { !identity(value) } pub fn main() { let _ = flip(False) Nil }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let function = &program.functions[0];
        let mut group = CallGroupCodegen::new(
            program.functions.iter().collect(),
            Vec::new(),
            BTreeSet::new(),
        );
        group.body_segments.insert(
            function.target.key(),
            BTreeMap::from([(0, 0), (1, 1), (2, 1)]),
        );
        let mut source = Code::default();
        group.write_body(&mut source, function);
        assert_eq!(
            source.as_str(),
            r#"enum Bool0State {
    Segment0(Bool0Segment0State),
    Segment1(Bool0Segment1State),
}
enum Bool0Segment0State {
    Point0 { bool0: bool },
}
enum Bool0Segment1State {
    Point1 { bool0: bool, bool1: bool },
    Point2 { bool0: bool, bool1: bool, bool2: bool },
}
fn calls_bool_0_run(mut active: Bool0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
    loop {
        let progress = match active {
            Bool0State::Segment0(active) => calls_bool_0_segment_0(active, ops, budget),
            Bool0State::Segment1(active) => calls_bool_0_segment_1(active, ops, budget),
        };
        match progress {
            ControlFlow::Continue(next) => active = next,
            ControlFlow::Break(step) => return step,
        }
    }
}
fn calls_bool_0_segment_0(active: Bool0Segment0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> ControlFlow<FunctionStep, Bool0State> {
    match active {
        Bool0Segment0State::Point0 { bool0 } => {
            if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Bool0Point0 { bool0 })); }
            *budget -= 1;
            {
                ControlFlow::Break(FunctionStep::BoolCall { callee: FunctionState::Bool1Point0 { bool0 }, caller: BoolReturn::Bool0Call0 { bool0 } })
            }
        },
    }
}
fn calls_bool_0_segment_1(active: Bool0Segment1State, _ops: &mut CallOps<'_>, budget: &mut usize) -> ControlFlow<FunctionStep, Bool0State> {
    match active {
        Bool0Segment1State::Point1 { bool0, bool1 } => {
            if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Bool0Point1 { bool0, bool1 })); }
            *budget -= 1;
            let bool2 = !bool1;
            if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Bool0Point2 { bool0, bool1, bool2 })); }
            *budget -= 1;
            {
                ControlFlow::Break(FunctionStep::Bool { value: bool2 })
            }
        },
        Bool0Segment1State::Point2 { bool0, bool1, bool2 } => {
            if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Bool0Point2 { bool0, bool1, bool2 })); }
            *budget -= 1;
            {
                ControlFlow::Break(FunctionStep::Bool { value: bool2 })
            }
        },
    }
}
"#
        );
    }

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

    #[test]
    fn interpreted_block_entries_have_local_handoff_without_unused_global_states() {
        let source = r#"
fn inspect(value: Int) -> Int {
  let callback = fn(value) { value }
  case value > 0 {
    True -> { echo value callback(value) }
    False -> callback(value + 1)
  }
}
pub fn main() { inspect(41) }
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
        let plan = &execution.execution;
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let mut interpreted_starts = Vec::new();
        for function in &program.functions {
            let group = CallGroupCodegen::new(vec![function], Vec::new(), BTreeSet::new());
            let external =
                CallGroupCodegen::new(vec![function], vec![function.target], BTreeSet::new());
            for &point in function.shape.starts.values() {
                if matches!(function.shape.points[point], CallPoint::Interpreted) {
                    interpreted_starts.push(point);
                    assert!(group.local_point(function, point));
                    assert!(!group.global_point(function, point));
                    assert!(external.global_point(function, point));
                }
            }
        }
        assert_eq!(interpreted_starts.len(), 1);
    }

    #[test]
    fn a_call_return_publishes_its_caller_before_an_interpreted_handoff() {
        let source = r#"
fn inspect(value: String) -> String {
  let callback = fn(value) { value }
  let returned = callback(value)
  echo returned
  returned
}
pub fn main() { inspect("x") <> "!" }
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
        let plan = &execution.execution;
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let mut handoffs = 0;
        for function in &program.functions {
            let group = CallGroupCodegen::new(vec![function], Vec::new(), BTreeSet::new());
            for call in &function.shape.calls {
                let point = call.point + 1;
                if matches!(function.shape.points[point], CallPoint::Interpreted) {
                    assert!(group.global_point(function, point));
                    assert!(group.local_point(function, point));
                    handoffs += 1;
                }
            }
        }
        assert_eq!(handoffs, 1);
    }

    #[test]
    fn segmented_scalar_flow_moves_locals_and_exits_the_current_loop() {
        let source = "pub fn choose(value: Float, flag: Bool) { let value = value +. 1.0 +. 2.0 case flag { True -> value False -> 0.0 } } pub fn main() { let result = choose(2.0, True) result +. 1.0 }";
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
        let plan = &execution.execution;
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let function = program
            .functions
            .iter()
            .find(|function| {
                function
                    .shape
                    .points
                    .iter()
                    .any(|point| matches!(point, CallPoint::Terminator(_)))
            })
            .unwrap();
        let mut group = CallGroupCodegen::new(
            program.functions.iter().collect(),
            Vec::new(),
            BTreeSet::new(),
        );
        // Exercise the helper grammar with short, valid checkpoint segments;
        // the long Float test separately owns the automatic size decision.
        group.body_segments.insert(
            function.target.key(),
            BTreeMap::from([
                (0, 0),
                (1, 1),
                (2, 1),
                (3, 3),
                (4, 3),
                (5, 5),
                (6, 6),
                (7, 6),
            ]),
        );
        let starts = segment_starts(function);
        let mut source = Code::default();
        group.write_body_match(
            &mut source,
            function,
            &[0],
            "Float1Segment0",
            false,
            &starts,
        );
        group.write_body_match(
            &mut source,
            function,
            &[1, 2],
            "Float1Segment1",
            true,
            &starts,
        );
        group.write_body_match(
            &mut source,
            function,
            &[3, 4],
            "Float1Segment3",
            true,
            &starts,
        );
        group
            .body_segments
            .get_mut(&function.target.key())
            .unwrap()
            .insert(4, 4);
        group.write_body_match(
            &mut source,
            function,
            &[4],
            "Float1Segment4",
            false,
            &starts,
        );
        assert_eq!(
            source.as_str(),
            r#"match active {
    Float1Segment0State::Point0 { float0, bool0 } => {
        if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Float1Point0 { float0, bool0 })); }
        *budget -= 1;
        let float1 = f64::from_bits(4607182418800017408);
        ControlFlow::Continue(Float1State::Segment1(Float1Segment1State::Point1 { float0, bool0, float1 }))
    },
}
match active {
    Float1Segment1State::Point1 { float0, bool0, float1 } => {
        if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Float1Point1 { float0, bool0, float1 })); }
        *budget -= 1;
        let float2 = float0 + float1;
        active = Float1Segment1State::Point2 { float0, bool0, float1, float2 };
        continue;
    },
    Float1Segment1State::Point2 { float0, bool0, float1, float2 } => {
        if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Float1Point2 { float0, bool0, float1, float2 })); }
        *budget -= 1;
        let float3 = f64::from_bits(4611686018427387904);
        return ControlFlow::Continue(Float1State::Segment3(Float1Segment3State::Point3 { float0, bool0, float1, float2, float3 }));
    },
}
match active {
    Float1Segment3State::Point3 { float0, bool0, float1, float2, float3 } => {
        if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Float1Point3 { float0, bool0, float1, float2, float3 })); }
        *budget -= 1;
        let float4 = float2 + float3;
        active = Float1Segment3State::Point4 { float0, bool0, float1, float2, float3, float4 };
        continue;
    },
    Float1Segment3State::Point4 { float0, bool0, float1, float2, float3, float4 } => {
        if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Float1Point4 { float0, bool0, float1, float2, float3, float4 })); }
        *budget -= 1;
        return ControlFlow::Continue({
            if bool0 { Float1State::Segment5(Float1Segment5State::Point5 { float0: float4 }) } else { Float1State::Segment6(Float1Segment6State::Point6 {  }) }
        });
    },
}
match active {
    Float1Segment4State::Point4 { float0, bool0, float1, float2, float3, float4 } => {
        if *budget == 0 { return ControlFlow::Break(FunctionStep::Yield(FunctionState::Float1Point4 { float0, bool0, float1, float2, float3, float4 })); }
        *budget -= 1;
        ControlFlow::Continue({
            if bool0 { Float1State::Segment5(Float1Segment5State::Point5 { float0: float4 }) } else { Float1State::Segment6(Float1Segment6State::Point6 {  }) }
        })
    },
}
"#
        );
    }
}
