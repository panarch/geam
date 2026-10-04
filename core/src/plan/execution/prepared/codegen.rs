mod bit_array;
mod custom;
mod custom_loop;
mod int_list;
pub(super) mod shape;

use self::shape::{
    CompiledBoolean, CompiledEdge, CompiledInstruction, CompiledTerminator, CompiledTest,
    KernelKind, NumericComparison, NumericInteger, NumericOperation,
};
use crate::plan::execution::compiled::{CompiledCheckpoint, CompiledLoopFunction};
use crate::plan::execution::function::{
    BoolFunctionId, ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionGraphProfile,
    ExecutionProfile, FunctionBodyOwner, FunctionExit, FunctionTables, IntFunctionId,
};
use crate::plan::execution::graph::{
    ArithmeticNode, ArithmeticOperand, BlockGraphExitId, BlockId, BoolLocalId, IntLocalId,
    IntegerOperand, ListLocal, MatchEdgeArgument, ParamLocal, ParamSlot, StorageFamily,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::type_::{CustomTypeTable, FunctionType, ValueType};
pub(super) use shape::CompiledShape;
use std::collections::BTreeSet;

pub(in crate::plan::execution::prepared) struct CompiledCodegen<'program, Profile: ExecutionProfile>
{
    functions: &'program FunctionTables<Profile>,
    custom_types: &'program CustomTypeTable,
}

struct FunctionCodegen<'graph, Graph: ExecutionGraphProfile> {
    name: String,
    shape: CompiledShape<'graph, Graph>,
}

struct TargetCodegen<'graph, Graph: ExecutionGraphProfile, Id> {
    function: Id,
    body: FunctionCodegen<'graph, Graph>,
}

struct CallbackCodegen<'graph, Graph: ExecutionGraphProfile, Id> {
    function: Id,
    body: FunctionCodegen<'graph, Graph>,
    returns: CallbackReturns,
}

enum CallbackReturns {
    Int(Vec<IntLocalId>),
    Bool(Vec<BoolLocalId>),
}

/// Rust text with explicit nesting. The caller owns every block boundary;
/// braces inside expressions or literals never change the indentation.
#[derive(Default)]
struct Code {
    text: String,
    indentation: usize,
    line_start: bool,
}

#[derive(Clone, Copy)]
enum ProgressOutput<'returns> {
    Direct,
    Resume,
    Callback(&'returns CallbackReturns),
}

impl<'program, Profile: ExecutionProfile> CompiledCodegen<'program, Profile> {
    pub(in crate::plan::execution::prepared) fn new(
        functions: &'program FunctionTables<Profile>,
        custom_types: &'program CustomTypeTable,
    ) -> Self {
        Self {
            functions,
            custom_types,
        }
    }
}

impl<Profile: ExecutionProfile> Emit for CompiledCodegen<'_, Profile> {
    fn emit(&self, output: &mut Rust) {
        let ints = self
            .functions
            .value_returns
            .int_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                let shape = CompiledShape::inspect(function.body())
                    .or_else(|| CompiledShape::inspect_bits(function.body()))
                    .or_else(|| {
                        CompiledShape::inspect_custom_loop(function.body(), self.custom_types)
                    })?;
                Some(TargetCodegen {
                    function: IntFunctionId(index),
                    body: FunctionCodegen {
                        name: format!("{}_int_{index}", shape.kind.name()),
                        shape,
                    },
                })
            })
            .collect::<Vec<_>>();
        let bools = self
            .functions
            .value_returns
            .bool_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                let shape = CompiledShape::inspect(function.body())
                    .or_else(|| CompiledShape::inspect_bits(function.body()))
                    .or_else(|| {
                        CompiledShape::inspect_custom_loop(function.body(), self.custom_types)
                    })?;
                Some(TargetCodegen {
                    function: BoolFunctionId(index),
                    body: FunctionCodegen {
                        name: format!("{}_bool_{index}", shape.kind.name()),
                        shape,
                    },
                })
            })
            .collect::<Vec<_>>();
        let customs = self
            .functions
            .value_returns
            .custom_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                let shape =
                    CompiledShape::inspect_bits(FunctionBodyOwner::function_body(function.body()))?;
                Some(TargetCodegen {
                    function: index,
                    body: FunctionCodegen {
                        name: format!("{}_custom_{index}", shape.kind.name()),
                        shape,
                    },
                })
            })
            .collect::<Vec<_>>();
        let int_lists = self
            .functions
            .list_returns
            .int_list_functions
            .iter()
            .filter_map(|(id, function)| {
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                let shape = CompiledShape::inspect(function.body())?;
                Some(TargetCodegen {
                    function: *id,
                    body: FunctionCodegen {
                        name: format!("{}_int_list_{}", shape.kind.name(), id.index),
                        shape,
                    },
                })
            })
            .collect::<Vec<_>>();
        if ints.is_empty() && bools.is_empty() && customs.is_empty() && int_lists.is_empty() {
            output.call("compiled::CompiledFunctions::interpreted", &[]);
            return;
        }
        let loops = ints
            .iter()
            .any(|function| function.body.shape.kind == KernelKind::CustomLoop)
            || bools
                .iter()
                .any(|function| function.body.shape.kind == KernelKind::CustomLoop);
        let callback_calls = ints
            .iter()
            .filter(|function| function.body.shape.kind == KernelKind::CustomLoop)
            .flat_map(|function| function.body.shape.loop_calls())
            .chain(
                bools
                    .iter()
                    .filter(|function| function.body.shape.kind == KernelKind::CustomLoop)
                    .flat_map(|function| function.body.shape.loop_calls()),
            )
            .map(|call| call.function)
            .collect::<Vec<_>>();
        let callback_ints = self
            .functions
            .value_returns
            .int_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                if !loops {
                    return None;
                }
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                if !callback_calls.iter().any(|call| matches!(call,
                    CompiledLoopFunction::Int { type_, .. } if callback_type_matches(function.entry().params(function.body()), type_))) {
                    return None;
                }
                let returns = function
                    .body()
                    .exits
                    .iter()
                    .map(|exit| match exit {
                        FunctionExit::Return(local) => Some(*local),
                        FunctionExit::TailCall { .. } => None,
                    })
                    .collect::<Option<Vec<_>>>()?;
                let shape = CompiledShape::inspect_callback(function.body(), self.custom_types)?;
                Some(CallbackCodegen {
                    function: IntFunctionId(index),
                    body: FunctionCodegen {
                        name: format!("{}_int_{index}", shape.kind.name()),
                        shape,
                    },
                    returns: CallbackReturns::Int(returns),
                })
            })
            .collect::<Vec<_>>();
        let callback_bools = self
            .functions
            .value_returns
            .bool_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                if !loops {
                    return None;
                }
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                if !callback_calls.iter().any(|call| matches!(call,
                    CompiledLoopFunction::Bool { type_, .. } if callback_type_matches(function.entry().params(function.body()), type_))) {
                    return None;
                }
                let returns = function
                    .body()
                    .exits
                    .iter()
                    .map(|exit| match exit {
                        FunctionExit::Return(local) => Some(*local),
                        FunctionExit::TailCall { .. } => None,
                    })
                    .collect::<Option<Vec<_>>>()?;
                let shape = CompiledShape::inspect_callback(function.body(), self.custom_types)?;
                Some(CallbackCodegen {
                    function: BoolFunctionId(index),
                    body: FunctionCodegen {
                        name: format!("{}_bool_{index}", shape.kind.name()),
                        shape,
                    },
                    returns: CallbackReturns::Bool(returns),
                })
            })
            .collect::<Vec<_>>();
        let mut source = Code::default();
        source.open("{\n");
        let resumes_next = ints
            .iter()
            .filter(|function| function.body.shape.kind != KernelKind::CustomLoop)
            .any(|function| function.body.resumes_next())
            || bools
                .iter()
                .filter(|function| function.body.shape.kind != KernelKind::CustomLoop)
                .any(|function| function.body.resumes_next())
            || customs.iter().any(|function| function.body.resumes_next())
            || int_lists
                .iter()
                .any(|function| function.body.resumes_next());
        let ordinary = ints
            .iter()
            .any(|function| function.body.shape.kind != KernelKind::CustomLoop)
            || bools
                .iter()
                .any(|function| function.body.shape.kind != KernelKind::CustomLoop)
            || !customs.is_empty()
            || !int_lists.is_empty();
        if ordinary {
            source.open("\nenum CompiledResume {\n");
            if resumes_next {
                source.push_str("Next(usize),\n");
            }
            source.push_str("Exit(data::compiled::CompiledProgress),\n");
            source.close("}\n");
        }
        if loops {
            source.open("\nenum CustomLoopResume {\n");
            source
                .push_str("Next(usize),\nExit(data::compiled::custom_loop::CustomLoopProgress),\n");
            source.close("}\n");
        }
        for callback in &callback_ints {
            callback.body.write_callback(&mut source, &callback.returns);
        }
        for callback in &callback_bools {
            callback.body.write_callback(&mut source, &callback.returns);
        }
        for function in &ints {
            function.body.write_code(
                &mut source,
                resumes_next || function.body.shape.kind == KernelKind::CustomLoop,
            );
        }
        for function in &bools {
            function.body.write_code(
                &mut source,
                resumes_next || function.body.shape.kind == KernelKind::CustomLoop,
            );
        }
        for function in &customs {
            function.body.write_code(&mut source, resumes_next);
        }
        for function in &int_lists {
            function.body.write_code(&mut source, resumes_next);
        }
        source.open("data::compiled::CompiledFunctions {\n");
        source.open("ints: data::Storage::Static(&[\n");
        for function in &ints {
            function
                .body
                .write_target(&mut source, &Rust::expression(&function.function));
        }
        source.close("]),\n");
        source.open("bools: data::Storage::Static(&[\n");
        for function in &bools {
            function
                .body
                .write_target(&mut source, &Rust::expression(&function.function));
        }
        source.close("]),\n");
        source.open("customs: data::Storage::Static(&[\n");
        for function in &customs {
            function
                .body
                .write_target(&mut source, &Rust::expression(&function.function));
        }
        source.close("]),\n");
        source.open("int_lists: data::Storage::Static(&[\n");
        for function in &int_lists {
            function
                .body
                .write_target(&mut source, &Rust::expression(&function.function));
        }
        source.close("]),\n");
        if loops {
            source.open("callbacks: data::compiled::CompiledCallbacks {\n");
            source.open("ints: data::Storage::Static(&[\n");
            for callback in &callback_ints {
                callback.body.write_callback_target(
                    &mut source,
                    &Rust::expression(&callback.function),
                    &callback.returns,
                );
            }
            source.close("]),\n");
            source.open("bools: data::Storage::Static(&[\n");
            for callback in &callback_bools {
                callback.body.write_callback_target(
                    &mut source,
                    &Rust::expression(&callback.function),
                    &callback.returns,
                );
            }
            source.close("]),\n");
            source.close("},\n");
        } else {
            source.push_str("callbacks: data::compiled::CompiledCallbacks::interpreted(),\n");
        }
        source.close("}\n");
        source.close("}");
        output.code(source.as_str());
    }
}

/// Compare source argument types, excluding the callee's separate capture
/// suffix. Only bodies usable by an admitted loop signature are generated.
fn callback_type_matches(params: &[ParamSlot], type_: &FunctionType) -> bool {
    params.len() == type_.arguments.len()
        && params
            .iter()
            .zip(type_.arguments.iter())
            .all(|(param, expected)| match (param.local(), expected) {
                (ParamLocal::Int(_), ValueType::Int) | (ParamLocal::Bool(_), ValueType::Bool) => {
                    true
                }
                (ParamLocal::Custom(local), ValueType::Custom(type_id)) => {
                    local.shape.type_id == *type_id
                }
                _ => false,
            })
}

impl<Graph: ExecutionGraphProfile> FunctionCodegen<'_, Graph> {
    fn write_code(&self, source: &mut Code, resumes_next: bool) {
        let name = &self.name;
        let resume = self.resume_type();
        let progress = self.progress_type();
        let checkpoints = self.shape.checkpoints.len();
        let values = self.shape.kind.values();
        let operations = self.shape.kind.operations_parameter();
        let argument = self.shape.kind.operations_argument();
        let resume_operations = self.shape.kind.operations_type();
        source.open(&format!(
            r#"
fn {name}(
    point: usize,
    values: &mut {values},
    {operations}budget: &mut usize,
) -> {progress} {{
"#
        ));
        source.open(&format!(
            r#"
const RESUME: [
    fn(&mut {values}, {resume_operations}&mut usize) -> {resume};
    {checkpoints}
] = [
"#
        ));
        for (index, point) in self.shape.checkpoints.iter().enumerate() {
            if index == self.entry() {
                if !matches!(
                    self.shape.kind,
                    KernelKind::IntList | KernelKind::CustomLoop
                ) {
                    let inputs = checkpoint_inputs(*point);
                    source.push_str(&format!(
                        "|values, budget| {resume}::Exit({}_entry({inputs}, values, budget)),\n",
                        self.name
                    ));
                } else {
                    source.open(&format!("|values{argument}, budget| {{\n"));
                    let inputs = self.take_inputs(source, *point);
                    source.push_str(&format!(
                        "{resume}::Exit({}_entry({inputs}, values{argument}, budget))\n",
                        self.name
                    ));
                    source.close("},\n");
                }
            } else {
                source.push_str(&format!("{},\n", self.resume_name(index)));
            }
        }
        source.close("];\n");
        if resumes_next {
            source.push_str(&format!(
                r#"
let mut point = point;
loop {{
    match RESUME[point](values{argument}, budget) {{
        {resume}::Next(next) => point = next,
        {resume}::Exit(progress) => return progress,
    }}
}}
"#,
            ));
        } else {
            source.push_str(&format!(
                "let {resume}::Exit(progress) = RESUME[point](values{argument}, budget);\nprogress\n"
            ));
        }
        source.close("}\n");
        let entry = self.shape.checkpoints[self.entry()];
        self.signature(source, &format!("{}_entry", self.name), entry, progress);
        let pattern = self.locals(entry, self.shape.repeats);
        source.push_str(&format!("let {pattern} = inputs;\n"));
        if self.shape.repeats {
            source.open("'repeat: loop {\n");
        }
        if self.shape.kind == KernelKind::BitArray {
            self.bit_loop(source);
        } else {
            self.path(source, entry.block, None, ProgressOutput::Direct);
        }
        if self.shape.repeats {
            source.close("}\n");
        }
        source.close("}\n");
        // A resumed checkpoint returns its next point to the dispatcher before
        // another step runs. Its Rust stack depth does not grow with the suffix.
        for (index, point) in self.shape.checkpoints.iter().enumerate() {
            if index == self.entry() {
                continue;
            }
            let name = self.resume_name(index);
            source.open(&format!(
                r#"
fn {name}(
    values: &mut {values},
    {operations}budget: &mut usize,
) -> {resume} {{
"#
            ));
            let inputs = self.take_inputs(source, *point);
            source.push_str(&format!(
                "let {} = {};\n",
                self.locals(*point, false),
                inputs
            ));
            let block = self.shape.block(point.block);
            if matches!(self.shape.kind, KernelKind::BitArray | KernelKind::Callback)
                && point.instruction == block.instructions.len()
                && matches!(block.terminator, CompiledTerminator::Interpreted)
            {
                source.push_str("let _ = budget;\n");
                self.resume_terminator(source, *point, &block.terminator);
                source.close("}\n");
                continue;
            }
            self.ready(source, index, ProgressOutput::Resume);
            if let Some(instruction) = block.instructions.get(point.instruction) {
                self.preflight(source, index, instruction, ProgressOutput::Resume);
                source.push_str("*budget -= 1;\n");
                self.instruction(source, *point, instruction, ProgressOutput::Resume);
                let next = self.shape.checkpoints[index + 1];
                self.big_exit(
                    source,
                    *point,
                    instruction,
                    index + 1,
                    ProgressOutput::Resume,
                );
                self.save(source, next);
                source.push_str(&format!("{resume}::Next({})\n", index + 1));
            } else {
                self.preflight_terminator(source, index, &block.terminator);
                if !matches!(
                    block.terminator,
                    CompiledTerminator::Interpreted
                        | CompiledTerminator::Match(_)
                        | CompiledTerminator::Custom(_)
                ) {
                    source.push_str("*budget -= 1;\n");
                }
                self.resume_terminator(source, *point, &block.terminator);
            }
            source.close("}\n");
        }
    }

    fn progress_type(&self) -> &'static str {
        if self.shape.kind == KernelKind::CustomLoop {
            "data::compiled::custom_loop::CustomLoopProgress"
        } else {
            "data::compiled::CompiledProgress"
        }
    }

    fn resume_type(&self) -> &'static str {
        if self.shape.kind == KernelKind::CustomLoop {
            "CustomLoopResume"
        } else {
            "CompiledResume"
        }
    }

    fn progress(&self, variant: &str, argument: &str) -> String {
        match self.shape.kind {
            KernelKind::CustomLoop => format!(
                "data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::{variant}({argument}))"
            ),
            KernelKind::Callback => format!(
                "data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::{variant}({argument}))"
            ),
            _ => format!("data::compiled::CompiledProgress::{variant}({argument})"),
        }
    }

    fn resumes_next(&self) -> bool {
        self.shape
            .checkpoints
            .iter()
            .enumerate()
            .any(|(index, point)| {
                if index == self.entry() {
                    return false;
                }
                let block = self.shape.block(point.block);
                point.instruction < block.instructions.len()
                    || !matches!(
                        block.terminator,
                        CompiledTerminator::Exit(_) | CompiledTerminator::Interpreted
                    )
            })
    }

    fn write_target(&self, source: &mut Code, function: &str) {
        let implementation = match self.shape.kind {
            KernelKind::Numeric => "Numeric",
            KernelKind::BitArray => "BitArray",
            KernelKind::IntList => "IntList",
            KernelKind::CustomLoop => "CustomLoop",
            KernelKind::Callback => return,
        };
        source.open("data::compiled::CompiledFunction {\n");
        source.push_str(&format!("function: {function},\n"));
        let storage = if self.shape.kind == KernelKind::CustomLoop {
            "data::Storage::Static(&"
        } else {
            ""
        };
        source.open(&format!(
            "implementation: data::compiled::CompiledImplementation::{implementation}({storage}data::compiled::{implementation}Implementation {{\n"
        ));
        source.push_str(&format!("entry: {},\n", self.entry()));
        source.open("checkpoints: data::Storage::Static(&[\n");
        for point in &self.shape.checkpoints {
            source.push_str(&format!("{},\n", Rust::expression(point)));
        }
        source.close("]),\n");
        if self.shape.kind == KernelKind::CustomLoop {
            source.open("calls: data::Storage::Static(&[\n");
            for call in self.shape.loop_calls() {
                source.push_str(&format!("{},\n", Rust::expression(&call)));
            }
            source.close("]),\n");
        }
        source.push_str(&format!("run: {},\n", self.name));
        source.close(if self.shape.kind == KernelKind::CustomLoop {
            "})),\n"
        } else {
            "}),\n"
        });
        source.close("},\n");
    }

    fn entry(&self) -> usize {
        self.shape.start(self.shape.graph.entry())
    }

    fn signature(&self, source: &mut Code, name: &str, point: CompiledCheckpoint, progress: &str) {
        let types = tuple(
            std::iter::repeat_n("i128".to_owned(), point.ints)
                .chain(std::iter::repeat_n("bool".to_owned(), point.bools))
                .chain(std::iter::repeat_n(
                    "data::compiled::bit_array::BitArrayRange".to_owned(),
                    point.bit_arrays,
                ))
                .chain(std::iter::repeat_n(
                    "data::compiled::int_list::IntList".to_owned(),
                    point.int_lists,
                ))
                .chain(std::iter::repeat_n(
                    "data::compiled::custom::CustomInput".to_owned(),
                    point.customs,
                ))
                .chain(std::iter::repeat_n(
                    "data::compiled::custom_loop::CustomList".to_owned(),
                    point.custom_lists,
                ))
                .chain(std::iter::repeat_n(
                    "data::compiled::custom_loop::IntCallback".to_owned(),
                    point.int_functions,
                ))
                .chain(std::iter::repeat_n(
                    "data::compiled::custom_loop::BoolCallback".to_owned(),
                    point.bool_functions,
                )),
        );
        let values = self.shape.kind.values();
        let operations = self.shape.kind.operations_parameter();
        source.open(&format!(
            r#"
fn {name}(
    inputs: {types},
    values: &mut {values},
    {operations}budget: &mut usize,
) -> {progress} {{
"#
        ));
    }

    fn locals(&self, point: CompiledCheckpoint, mutable: bool) -> String {
        let prefix = if mutable { "mut " } else { "" };
        tuple(
            (0..point.ints)
                .map(|index| format!("{prefix}b{}_i{index}", point.block.0))
                .chain((0..point.bools).map(|index| format!("{prefix}b{}_v{index}", point.block.0)))
                .chain(
                    (0..point.bit_arrays)
                        .map(|index| format!("{prefix}b{}_b{index}", point.block.0)),
                )
                .chain(
                    (0..point.int_lists)
                        .map(|index| format!("{prefix}b{}_l{index}", point.block.0)),
                )
                .chain(
                    (0..point.customs).map(|index| format!("{prefix}b{}_c{index}", point.block.0)),
                )
                .chain(
                    (0..point.custom_lists)
                        .map(|index| format!("{prefix}b{}_l{index}", point.block.0)),
                )
                .chain(
                    (0..point.int_functions)
                        .map(|index| format!("{prefix}b{}_f{index}", point.block.0)),
                )
                .chain(
                    (0..point.bool_functions)
                        .map(|index| format!("{prefix}b{}_g{index}", point.block.0)),
                ),
        )
    }

    fn resume_name(&self, point: usize) -> String {
        format!("{}_resume_{point}", self.name)
    }

    fn tick(&self, source: &mut Code, index: usize, output: ProgressOutput<'_>) {
        self.ready(source, index, output);
        source.push_str("*budget -= 1;\n");
    }

    fn take_inputs(&self, source: &mut Code, point: CompiledCheckpoint) -> String {
        // Reverse removal always removes the last remaining handle. Each
        // actual prefix moves once; no cloning or front-removal shifts.
        for index in (0..point.int_lists).rev() {
            source.push_str(&format!(
                "let _list{index} = values.int_lists.remove({index});\n"
            ));
        }
        for index in (0..point.customs).rev() {
            source.push_str(&format!(
                "let _custom{index} = values.customs.remove({index});\n"
            ));
        }
        for (field, prefix, count) in [
            ("custom_lists", "custom_list", point.custom_lists),
            ("int_functions", "int_function", point.int_functions),
            ("bool_functions", "bool_function", point.bool_functions),
        ] {
            for index in (0..count).rev() {
                source.push_str(&format!(
                    "let _{prefix}{index} = values.{field}.remove({index});\n"
                ));
            }
        }
        tuple(
            (0..point.ints)
                .map(|index| format!("values.ints[{index}]"))
                .chain((0..point.bools).map(|index| format!("values.bools[{index}]")))
                .chain((0..point.bit_arrays).map(|index| format!("values.bit_arrays[{index}]")))
                .chain((0..point.int_lists).map(|index| format!("_list{index}")))
                .chain((0..point.customs).map(|index| format!("_custom{index}")))
                .chain((0..point.custom_lists).map(|index| format!("_custom_list{index}")))
                .chain((0..point.int_functions).map(|index| format!("_int_function{index}")))
                .chain((0..point.bool_functions).map(|index| format!("_bool_function{index}"))),
        )
    }

    fn edge_inputs(&self, source: &mut Code, from: BlockId, edge: CompiledEdge<'_>) -> String {
        let parameters = self.shape.graph.block(edge.target()).params();
        let inputs = match edge {
            CompiledEdge::Ordinary(edge) => edge
                .args()
                .iter()
                .filter_map(|local| local_expression(from, local))
                .collect::<Vec<_>>(),
            CompiledEdge::Match(edge) => edge
                .args()
                .iter()
                .zip(parameters)
                .filter_map(|(argument, parameter)| match argument {
                    MatchEdgeArgument::Value(local) => local_expression(from, local),
                    MatchEdgeArgument::Binding(index) => {
                        if let CompiledTerminator::Match(view) = &self.shape.block(from).terminator
                            && view
                                .lists
                                .iter()
                                .any(|&(binding, tail)| binding == *index && tail.is_none())
                        {
                            return local_expression(from, view.matcher.subject());
                        }
                        if let CompiledTerminator::Custom(view) = &self.shape.block(from).terminator
                            && view.aliases.contains(index)
                        {
                            return local_expression(from, view.matcher.subject());
                        }
                        parameter
                            .local()
                            .storage_slot()
                            .map(|slot| (slot.family, format!("m{index}")))
                    }
                })
                .collect(),
        };
        let ints = inputs
            .iter()
            .filter(|(family, _)| *family == StorageFamily::Int)
            .map(|(_, expression)| expression.clone());
        let bools = inputs
            .iter()
            .filter(|(family, _)| *family == StorageFamily::Bool)
            .map(|(_, expression)| expression.clone());
        let point = self.shape.checkpoints
            [self.shape.start(from) + self.shape.block(from).instructions.len()];
        if self.shape.kind == KernelKind::CustomLoop {
            return self.loop_edge_inputs(source, from, inputs, point);
        }
        let (family, prefix, count) = match self.shape.kind {
            KernelKind::IntList => (StorageFamily::IntList, "l", point.int_lists),
            KernelKind::Callback => (StorageFamily::Custom, "c", point.customs),
            _ => {
                let bits = inputs
                    .iter()
                    .filter(|(family, _)| *family == StorageFamily::BitArray)
                    .map(|(_, expression)| expression.clone());
                return tuple(ints.chain(bools).chain(bits));
            }
        };
        let mut owners = inputs
            .iter()
            .filter(|(input_family, _)| *input_family == family)
            .map(|(_, expression)| expression.clone())
            .collect::<Vec<_>>();
        let mut uses = BTreeSet::new();
        // The last edge use moves the handle; preceding uses retain it.
        for expression in owners.iter_mut().rev() {
            if !uses.insert(expression.clone()) {
                *expression = format!("{expression}.clone()");
            }
        }
        let inputs = tuple(ints.chain(bools).chain(owners));
        let omits_owner =
            (0..count).any(|index| !uses.contains(&format!("b{}_{prefix}{index}", from.0)));
        if !omits_owner {
            return inputs;
        }
        source.push_str(&format!("let _next = {inputs};\n"));
        for index in 0..count {
            let name = format!("b{}_{prefix}{index}", from.0);
            if !uses.contains(&name) {
                source.push_str(&format!("drop({name});\n"));
            }
        }
        "_next".to_owned()
    }

    fn interpreted(
        &self,
        source: &mut Code,
        index: usize,
        returning: bool,
        output: ProgressOutput<'_>,
    ) {
        self.save(source, self.shape.checkpoints[index]);
        let (prefix, suffix) = if returning {
            ("return ", ";")
        } else {
            ("", "")
        };
        let progress = output.for_kind(
            self.progress("Interpreted", &index.to_string()),
            self.shape.kind,
        );
        source.push_str(&format!("{prefix}{progress}{suffix}\n"));
    }

    fn ready(&self, source: &mut Code, index: usize, output: ProgressOutput<'_>) {
        source.open("if *budget == 0 {\n");
        self.save(source, self.shape.checkpoints[index]);
        let progress = output.for_kind(self.progress("Yield", &index.to_string()), self.shape.kind);
        source.push_str(&format!("return {progress};\n"));
        source.close("}\n");
    }

    fn save(&self, source: &mut Code, point: CompiledCheckpoint) {
        let ints = (0..point.ints)
            .map(|index| format!("b{}_i{index}", point.block.0))
            .collect::<Vec<_>>()
            .join(", ");
        let bools = (0..point.bools)
            .map(|index| format!("b{}_v{index}", point.block.0))
            .collect::<Vec<_>>()
            .join(", ");
        source.push_str(&format!(
            "\nvalues.ints.clear();\nvalues.ints.extend_from_slice(&[{ints}]);\n"
        ));
        source.push_str(&format!(
            "values.bools.clear();\nvalues.bools.extend_from_slice(&[{bools}]);\n"
        ));
        if self.shape.kind == KernelKind::BitArray {
            let ranges = (0..point.bit_arrays)
                .map(|index| format!("b{}_b{index}", point.block.0))
                .collect::<Vec<_>>()
                .join(", ");
            source.push_str(&format!(
                "values.bit_arrays.clear();\nvalues.bit_arrays.extend_from_slice(&[{ranges}]);\n"
            ));
        }
        let owner = match self.shape.kind {
            KernelKind::IntList => Some(("int_lists", "l", point.int_lists)),
            KernelKind::Callback => Some(("customs", "c", point.customs)),
            _ => None,
        };
        if let Some((field, prefix, count)) = owner {
            let owners = (0..count)
                .map(|index| format!("b{}_{prefix}{index}", point.block.0))
                .collect::<Vec<_>>()
                .join(", ");
            source.push_str(&format!(
                "values.{field}.clear();\nvalues.{field}.extend([{owners}]);\n"
            ));
        }
        if self.shape.kind == KernelKind::CustomLoop {
            for (field, prefix, count) in [
                ("customs", "c", point.customs),
                ("custom_lists", "l", point.custom_lists),
                ("int_functions", "f", point.int_functions),
                ("bool_functions", "g", point.bool_functions),
            ] {
                let owners = (0..count)
                    .map(|index| format!("b{}_{prefix}{index}", point.block.0))
                    .collect::<Vec<_>>()
                    .join(", ");
                source.push_str(&format!(
                    "values.{field}.clear();\nvalues.{field}.extend([{owners}]);\n"
                ));
            }
        }
    }

    fn path(
        &self,
        source: &mut Code,
        block: BlockId,
        stop: Option<BlockId>,
        output: ProgressOutput<'_>,
    ) {
        let body = self.shape.block(block);
        for (instruction_index, instruction) in body.instructions.iter().enumerate() {
            let index = self.shape.start(block) + instruction_index;
            self.ready(source, index, output);
            self.preflight(source, index, instruction, output);
            source.push_str("*budget -= 1;\n");
            self.instruction(source, self.shape.checkpoints[index], instruction, output);
            self.big_exit(
                source,
                self.shape.checkpoints[index],
                instruction,
                index + 1,
                output,
            );
        }
        let index = self.shape.start(block) + body.instructions.len();
        let point = self.shape.checkpoints[index];
        self.ready(source, index, output);
        self.preflight_terminator(source, index, &body.terminator);
        if !matches!(
            body.terminator,
            CompiledTerminator::Interpreted
                | CompiledTerminator::Match(_)
                | CompiledTerminator::Custom(_)
        ) {
            source.push_str("*budget -= 1;\n");
        }
        let join = self.shape.joins[block.0];
        if let Some(join) = join {
            let join_point = self.shape.checkpoints[self.shape.start(join)];
            source.push_str(&format!("let {} = ", self.locals(join_point, false)));
        }
        self.branch(
            source,
            point,
            &body.terminator,
            self.shape.repeats || join.is_some() || stop.is_some(),
            output,
            &mut |source, target, inputs| self.edge(source, target, inputs, join.or(stop), output),
        );
        if let Some(join) = join {
            source.finish_statement();
            if Some(join) == stop {
                source
                    .push_str(&self.locals(self.shape.checkpoints[self.shape.start(join)], false));
                source.push_str("\n");
            } else {
                self.path(source, join, stop, output);
            }
        }
    }

    fn edge(
        &self,
        source: &mut Code,
        target: BlockId,
        inputs: String,
        stop: Option<BlockId>,
        output: ProgressOutput<'_>,
    ) {
        if target == self.shape.graph.entry() {
            source.push_str(&format!(
                "{} = {inputs};\ncontinue 'repeat;\n",
                self.locals(self.shape.checkpoints[self.entry()], false)
            ));
        } else if Some(target) == stop {
            source.push_str(&inputs);
            source.push_str("\n");
        } else {
            let point = self.shape.checkpoints[self.shape.start(target)];
            source.push_str(&format!("let {} = {inputs};\n", self.locals(point, false)));
            self.path(source, target, stop, output);
        }
    }

    fn branch(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        terminator: &CompiledTerminator<'_>,
        returning: bool,
        output: ProgressOutput<'_>,
        mut emit_edge: &mut dyn FnMut(&mut Code, BlockId, String),
    ) {
        match terminator {
            CompiledTerminator::Jump(edge) => {
                source.open("{\n");
                let inputs = self.edge_inputs(source, point.block, CompiledEdge::Ordinary(edge));
                emit_edge(source, edge.target(), inputs);
                source.close("}\n");
            }
            CompiledTerminator::Boolean {
                subject,
                true_,
                false_,
            } => {
                source.open(&format!("if b{}_v{} {{\n", point.block.0, subject.0));
                let inputs = self.edge_inputs(source, point.block, CompiledEdge::Ordinary(true_));
                emit_edge(source, true_.target(), inputs);
                source.alternative("} else {\n");
                let inputs = self.edge_inputs(source, point.block, CompiledEdge::Ordinary(false_));
                emit_edge(source, false_.target(), inputs);
                source.close("}\n");
            }
            CompiledTerminator::Test {
                test,
                true_,
                false_,
            } => {
                source.open(&format!("if {} {{\n", test_expression(point.block, test)));
                let inputs = self.edge_inputs(source, point.block, CompiledEdge::Ordinary(true_));
                emit_edge(source, true_.target(), inputs);
                source.alternative("} else {\n");
                let inputs = self.edge_inputs(source, point.block, CompiledEdge::Ordinary(false_));
                emit_edge(source, false_.target(), inputs);
                source.close("}\n");
            }
            CompiledTerminator::Switch {
                subject,
                clauses,
                fallback,
            } => {
                for (literal, edge) in clauses.iter() {
                    source.open(&format!(
                        "if b{}_i{} == {}_i128 {{\n",
                        point.block.0, subject.0, literal
                    ));
                    let inputs =
                        self.edge_inputs(source, point.block, CompiledEdge::Ordinary(edge));
                    emit_edge(source, edge.target(), inputs);
                    source.close("} else ");
                }
                source.open("{\n");
                let inputs =
                    self.edge_inputs(source, point.block, CompiledEdge::Ordinary(fallback));
                emit_edge(source, fallback.target(), inputs);
                source.close("}\n");
            }
            CompiledTerminator::Exit(exit) => {
                self.complete(source, point, *exit, returning, output)
            }
            CompiledTerminator::Match(view) => {
                self.match_branch(
                    source,
                    point,
                    view,
                    returning,
                    output,
                    &mut |source, edge| {
                        let inputs = self.edge_inputs(source, point.block, edge);
                        emit_edge(source, edge.target(), inputs);
                    },
                );
            }
            CompiledTerminator::Custom(view) => {
                self.custom_match_branch(
                    source,
                    point,
                    view,
                    returning,
                    output,
                    &mut |source, edge| {
                        let inputs = self.edge_inputs(source, point.block, edge);
                        emit_edge(source, edge.target(), inputs);
                    },
                );
            }
            CompiledTerminator::BitArray(matcher) => {
                self.bit_match(source, point, matcher, output, &mut emit_edge)
            }
            CompiledTerminator::Interpreted => self.interpreted(
                source,
                self.shape.start(point.block) + point.instruction,
                returning,
                output,
            ),
        }
    }

    fn resume_terminator(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        terminator: &CompiledTerminator<'_>,
    ) {
        self.branch(
            source,
            point,
            terminator,
            false,
            ProgressOutput::Resume,
            &mut |source, target, inputs| {
                let next = self.shape.start(target);
                let target = self.shape.checkpoints[next];
                source.push_str(&format!("let {} = {inputs};\n", self.locals(target, false)));
                self.save(source, target);
                source.push_str(&format!("{}::Next({next})\n", self.resume_type()));
            },
        );
    }

    fn complete(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        exit: BlockGraphExitId,
        returning: bool,
        output: ProgressOutput<'_>,
    ) {
        if let ProgressOutput::Callback(returns) = output {
            let value = match returns {
                CallbackReturns::Int(returns) => {
                    format!("b{}_i{}", point.block.0, returns[exit.0].0)
                }
                CallbackReturns::Bool(returns) => {
                    format!("b{}_v{}", point.block.0, returns[exit.0].0)
                }
            };
            let prefix = if returning { "return " } else { "" };
            let suffix = if returning { ";" } else { "" };
            source.push_str(&format!(
                "{prefix}data::compiled::custom_loop::CallbackProgress::Complete({value}){suffix}\n"
            ));
            return;
        }
        self.save(source, point);
        let (prefix, suffix) = if returning {
            ("return ", ";")
        } else {
            ("", "")
        };
        let progress = output.for_kind(
            self.progress(
                "Complete",
                &format!("data::graph::BlockGraphExitId({})", exit.0),
            ),
            self.shape.kind,
        );
        source.push_str(&format!("{prefix}{progress}{suffix}\n"));
    }

    fn preflight(
        &self,
        source: &mut Code,
        index: usize,
        instruction: &CompiledInstruction<'_>,
        output: ProgressOutput<'_>,
    ) {
        match instruction {
            CompiledInstruction::CustomLoop(instruction) => {
                self.loop_preflight(source, index, instruction, output)
            }
            CompiledInstruction::CustomField(field) => {
                self.custom_preflight(source, index, field, output)
            }
            _ => self.list_preflight(source, index, instruction, output),
        }
    }

    fn preflight_terminator(
        &self,
        source: &mut Code,
        index: usize,
        terminator: &CompiledTerminator<'_>,
    ) {
        match terminator {
            CompiledTerminator::Custom(view) => {
                self.custom_preflight_terminator(source, self.shape.checkpoints[index], view)
            }
            _ => self.list_preflight_terminator(source, index, terminator),
        }
    }

    fn instruction(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        instruction: &CompiledInstruction<'_>,
        output: ProgressOutput<'_>,
    ) {
        match instruction {
            CompiledInstruction::Integer(output, expression) => {
                source.push_str(&format!(
                    "let b{}_i{} = {};\n",
                    point.block.0,
                    output.0,
                    int_expression(point.block, expression)
                ));
            }
            CompiledInstruction::Boolean(output, expression) => {
                let expression = match expression {
                    CompiledBoolean::Value(value) => value.to_string(),
                    CompiledBoolean::Test(test) => test_expression(point.block, test),
                };
                source.push_str(&format!(
                    "let b{}_v{} = {expression};\n",
                    point.block.0, output.0
                ));
            }
            CompiledInstruction::Region { region, outputs } => {
                for (index, node) in region.nodes.iter().enumerate() {
                    let operand = |operand| match operand {
                        ArithmeticOperand::Input(index) => {
                            format!("b{}_i{}", point.block.0, region.inputs[index].0)
                        }
                        ArithmeticOperand::Value(index) => {
                            format!("_r{}_n{index}", point.instruction)
                        }
                        ArithmeticOperand::Immediate(value) => format!("{value}_i128"),
                    };
                    let expression = match *node {
                        ArithmeticNode::Add(left, right) => {
                            format!("{} + {}", operand(left), operand(right))
                        }
                        ArithmeticNode::Subtract(left, right) => {
                            format!("{} - {}", operand(left), operand(right))
                        }
                        ArithmeticNode::Multiply(left, right) => {
                            format!("{} * {}", operand(left), operand(right))
                        }
                        ArithmeticNode::Divide(left, right) => {
                            division(operand(left), operand(right), "/")
                        }
                        ArithmeticNode::Remainder(left, right) => {
                            division(operand(left), operand(right), "%")
                        }
                        ArithmeticNode::Negate(value) => format!("-{}", operand(value)),
                    };
                    source.push_str(&format!(
                        "let _r{}_n{index} = {expression};\n",
                        point.instruction
                    ));
                }
                for (output, local) in region.outputs.iter().zip(outputs) {
                    source.push_str(&format!(
                        "let b{}_i{} = _r{}_n{};\n",
                        point.block.0, local.0, point.instruction, output.value
                    ));
                }
            }
            CompiledInstruction::CustomLoop(instruction) => {
                self.loop_instruction(source, point, instruction, output)
            }
            CompiledInstruction::IntList(instruction) => {
                self.list_instruction(source, point, instruction)
            }
            // The scalar field read is staged before charging its step.
            CompiledInstruction::CustomField(_) => {}
        }
    }

    fn big_exit(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        instruction: &CompiledInstruction<'_>,
        next: usize,
        output: ProgressOutput<'_>,
    ) {
        // A Small remainder (including zero and MIN % -1) stays Small;
        // admitted literals and Boolean outputs likewise need no Big exit.
        let outputs = match instruction {
            CompiledInstruction::Integer(
                _,
                NumericInteger::Value(_) | NumericInteger::Binary(NumericOperation::Remainder, ..),
            )
            | CompiledInstruction::Boolean(..)
            | CompiledInstruction::IntList(_)
            | CompiledInstruction::CustomField(_)
            | CompiledInstruction::CustomLoop(_) => return,
            CompiledInstruction::Integer(output, _) => std::slice::from_ref(output),
            CompiledInstruction::Region { outputs, .. } => outputs.as_slice(),
        };
        let checks = outputs
            .iter()
            .map(|local| {
                let name = format!("b{}_i{}", point.block.0, local.0);
                format!("{name} < i128::from(i64::MIN) || {name} > i128::from(i64::MAX)")
            })
            .collect::<Vec<_>>();
        if checks.is_empty() {
            return;
        }
        source.open(&format!("if {} {{\n", checks.join(" || ")));
        self.save(source, self.shape.checkpoints[next]);
        let progress = output.for_kind(
            self.progress("Interpreted", &next.to_string()),
            self.shape.kind,
        );
        source.push_str(&format!("return {progress};\n"));
        source.close("}\n");
    }
}

impl ProgressOutput<'_> {
    fn for_kind(self, progress: String, kind: KernelKind) -> String {
        match self {
            Self::Direct | Self::Callback(_) => progress,
            Self::Resume if kind == KernelKind::CustomLoop => {
                format!("CustomLoopResume::Exit({progress})")
            }
            Self::Resume => format!("CompiledResume::Exit({progress})"),
        }
    }
}

impl Code {
    fn as_str(&self) -> &str {
        &self.text
    }

    fn push_str(&mut self, text: &str) {
        for line in text.split_inclusive('\n') {
            if self.line_start && line != "\n" {
                for _ in 0..self.indentation {
                    self.text.push_str("    ");
                }
            }
            self.text.push_str(line);
            self.line_start = line.ends_with('\n');
        }
    }

    fn open(&mut self, text: &str) {
        self.push_str(text);
        self.indentation += 1;
    }

    fn close(&mut self, text: &str) {
        self.indentation -= 1;
        self.push_str(text);
    }

    fn alternative(&mut self, text: &str) {
        self.close(text);
        self.indentation += 1;
    }

    /// A structured branch has just closed its expression and ended the line.
    fn finish_statement(&mut self) {
        self.text.truncate(self.text.len() - 1);
        self.line_start = false;
        self.push_str(";\n");
    }
}

fn checkpoint_inputs(point: CompiledCheckpoint) -> String {
    tuple(
        (0..point.ints)
            .map(|index| format!("values.ints[{index}]"))
            .chain((0..point.bools).map(|index| format!("values.bools[{index}]")))
            .chain((0..point.bit_arrays).map(|index| format!("values.bit_arrays[{index}]"))),
    )
}

fn tuple(values: impl IntoIterator<Item = String>) -> String {
    let values = values
        .into_iter()
        .map(|value| format!("{value},"))
        .collect::<Vec<_>>()
        .join(" ");
    format!("({values})")
}

fn local_expression(block: BlockId, local: &ParamLocal) -> Option<(StorageFamily, String)> {
    Some(match local {
        ParamLocal::Int(local) => (StorageFamily::Int, format!("b{}_i{}", block.0, local.0)),
        ParamLocal::Bool(local) => (StorageFamily::Bool, format!("b{}_v{}", block.0, local.0)),
        ParamLocal::BitArray(local) => (
            StorageFamily::BitArray,
            format!("b{}_b{}", block.0, local.0),
        ),
        ParamLocal::List(ListLocal::Int { local, .. }) => {
            (StorageFamily::IntList, format!("b{}_l{}", block.0, local.0))
        }
        ParamLocal::Custom(local) => (
            StorageFamily::Custom,
            format!("b{}_c{}", block.0, local.id().0),
        ),
        ParamLocal::List(ListLocal::Custom { local, .. }) => (
            StorageFamily::CustomList,
            format!("b{}_l{}", block.0, local.0),
        ),
        ParamLocal::IntFunction { local, .. } => (
            StorageFamily::IntFunction,
            format!("b{}_f{}", block.0, local.0),
        ),
        ParamLocal::BoolFunction { local, .. } => (
            StorageFamily::BoolFunction,
            format!("b{}_g{}", block.0, local.0),
        ),
        _ => return None,
    })
}

impl KernelKind {
    fn name(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::IntList => "int_list",
            Self::BitArray => "bit_array",
            Self::CustomLoop => "custom_loop",
            Self::Callback => "callback",
        }
    }
    fn values(self) -> &'static str {
        match self {
            Self::Numeric => "data::compiled::numeric::NumericValues",
            Self::IntList => "data::compiled::int_list::IntListValues",
            Self::BitArray => "data::compiled::bit_array::BitArrayValues",
            Self::Callback => "data::compiled::custom::CustomValues",
            Self::CustomLoop => "data::compiled::custom_loop::CustomLoopValues",
        }
    }
    fn operations_parameter(self) -> &'static str {
        match self {
            Self::Numeric | Self::BitArray | Self::Callback => "",
            Self::IntList => "_lists: &data::compiled::int_list::IntListOps<'_>,\n    ",
            Self::CustomLoop => "_lists: &data::compiled::custom_loop::CustomListOps<'_>,\n    ",
        }
    }
    fn operations_argument(self) -> &'static str {
        match self {
            Self::Numeric | Self::BitArray | Self::Callback => "",
            Self::IntList | Self::CustomLoop => ", _lists",
        }
    }
    fn operations_type(self) -> &'static str {
        match self {
            Self::Numeric | Self::BitArray | Self::Callback => "",
            Self::IntList => "&data::compiled::int_list::IntListOps<'_>, ",
            Self::CustomLoop => "&data::compiled::custom_loop::CustomListOps<'_>, ",
        }
    }
}

fn operand_expression(block: BlockId, operand: IntegerOperand) -> String {
    match operand {
        IntegerOperand::Local(local) => format!("b{}_i{}", block.0, local.0),
        IntegerOperand::Immediate(value) => format!("{value}_i128"),
    }
}

fn int_expression(block: BlockId, expression: &NumericInteger<'_>) -> String {
    match expression {
        NumericInteger::Value(value) => format!("{value}_i128"),
        NumericInteger::Negate(value) => format!("-b{}_i{}", block.0, value.0),
        NumericInteger::Binary(operation, left, right) => {
            let left = operand_expression(block, *left);
            let right = operand_expression(block, *right);
            match operation {
                NumericOperation::Add => format!("{left} + {right}"),
                NumericOperation::Subtract => format!("{left} - {right}"),
                NumericOperation::Multiply => format!("{left} * {right}"),
                NumericOperation::Divide => division(left, right, "/"),
                NumericOperation::Remainder => division(left, right, "%"),
            }
        }
    }
}

fn division(left: String, right: String, operator: &str) -> String {
    if right == "0_i128" {
        "0_i128".to_owned()
    } else {
        format!("if {right} == 0 {{ 0_i128 }} else {{ {left} {operator} {right} }}")
    }
}

fn test_expression(block: BlockId, test: &CompiledTest) -> String {
    match test {
        CompiledTest::Not(value) => format!("!b{}_v{}", block.0, value.0),
        CompiledTest::Compare(comparison, left, right) => {
            let operator = match comparison {
                NumericComparison::Equal => "==",
                NumericComparison::NotEqual => "!=",
                NumericComparison::Less => "<",
                NumericComparison::LessEqual => "<=",
                NumericComparison::Greater => ">",
                NumericComparison::GreaterEqual => ">=",
            };
            format!(
                "{} {operator} {}",
                operand_expression(block, *left),
                operand_expression(block, *right)
            )
        }
        CompiledTest::IntList(test) => int_list::test_expression(block, test),
        CompiledTest::CustomListLength {
            list,
            length,
            at_least,
        } => length_expression(&format!("b{}_l{}", block.0, list.0), *length, *at_least),
    }
}

fn length_expression(subject: &str, length: usize, at_least: bool) -> String {
    match (length, at_least) {
        (0, true) => "true".to_owned(),
        (0, false) => format!("{subject}.is_empty()"),
        (1, true) => format!("!{subject}.is_empty()"),
        (_, true) => format!("{subject}.len() >= {length}"),
        (_, false) => format!("{subject}.len() == {length}"),
    }
}

#[cfg(test)]
mod tests {
    pub(super) fn emit_edge_expression(
        source: &mut super::Code,
        _: super::BlockId,
        inputs: String,
    ) {
        source.push_str(&inputs);
        source.push_str("\n");
    }

    use super::int_list::IntListTest;
    use super::{
        Code, CompiledCodegen, CompiledShape, CompiledTerminator, CompiledTest, FunctionCodegen,
        NumericComparison, NumericInteger, NumericOperation, ProgressOutput, Rust, int_expression,
        local_expression, test_expression,
    };
    use crate::embedding::{BigInt, FunctionDeclaration, ModuleBuilder};
    use crate::plan::execution::function::{
        BoolFunctionId, ExecutionIntFunctionBody, FunctionExit, IntFunctionId,
    };
    use crate::plan::execution::graph::{
        ArithmeticNode, ArithmeticOperand, ArithmeticOutput, ArithmeticRegion, BlockGraphExitId,
        BlockId, BoolBranch, BoolLocalId, BoolTest, CustomListLocalId, Edge, FamilyTransfer,
        FloatLocalId, IntListLocalId, IntLocalId, IntSwitch, IntegerLiteral, IntegerOperand, Jump,
        ListLocal, ParamLocal, ParamSlot, ProfiledBlock, ProfiledBlockGraph, StorageFamily,
        Terminator, TestBranch, Transfer, TransferStep,
    };
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
    use std::convert::Infallible;

    #[test]
    fn custom_list_conditions_emit_empty_and_fixed_length_tests() {
        for (length, at_least, expected) in [
            (0, false, "b2_l3.is_empty()"),
            (0, true, "true"),
            (1, true, "!b2_l3.is_empty()"),
            (2, false, "b2_l3.len() == 2"),
            (2, true, "b2_l3.len() >= 2"),
        ] {
            assert_eq!(
                test_expression(
                    BlockId(2),
                    &CompiledTest::CustomListLength {
                        list: CustomListLocalId(3),
                        length,
                        at_least,
                    }
                ),
                expected,
            );
        }
    }

    #[test]
    fn integer_return_keeps_a_boolean_literal_before_shared_branches() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
fn choose(first: Int, second: Int) -> Int {
  let flag = True
  case first {
    0 -> case flag {
      True -> second
      False -> first
    }
    _ -> case flag {
      True -> first
      False -> second
    }
  }
}

pub fn main() { choose(3, 8) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let id = IntFunctionId(1);
        let function = FunctionCodegen {
            name: "numeric_int_1".to_owned(),
            shape: CompiledShape::inspect(plan.int_function(id).body()).unwrap(),
        };
        let mut source = Code::default();
        let point = function.shape.checkpoints[0];
        function.instruction(
            &mut source,
            point,
            &function.shape.block(point.block).instructions[0],
            ProgressOutput::Direct,
        );
        assert_eq!(source.as_str(), "let b0_v0 = true;\n");
    }

    #[test]
    fn block_locals_emit_only_the_supported_typed_families() {
        for (local, expected) in [
            (
                ParamLocal::Int(IntLocalId(2)),
                Some((StorageFamily::Int, "b3_i2")),
            ),
            (
                ParamLocal::Bool(BoolLocalId(1)),
                Some((StorageFamily::Bool, "b3_v1")),
            ),
            (ParamLocal::Float(FloatLocalId(0)), None),
        ] {
            assert_eq!(
                local_expression(BlockId(3), &local),
                expected.map(|(family, expression)| (family, expression.to_owned()))
            );
        }
    }

    #[test]
    fn list_checkpoint_inputs_move_once_and_save_only_the_actual_typed_prefix() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
fn choose(left: List(Int), right: List(Int), flag: Bool, value: Int) -> Int {
  case flag {
    True -> {
      let assert [first, ..] = left
      value + first
    }
    False -> {
      let assert [first, ..] = right
      value - first
    }
  }
}
pub fn main() { choose([7], [2], True, 3) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let point = shape.checkpoints[shape.start(shape.graph.entry())];
        assert_eq!(point.block, BlockId(0));
        assert_eq!((point.ints, point.bools, point.int_lists), (1, 1, 2));
        let function = FunctionCodegen {
            name: "choose".into(),
            shape,
        };
        let mut inputs = Code::default();
        assert_eq!(
            function.take_inputs(&mut inputs, point),
            "(values.ints[0], values.bools[0], _list0, _list1,)"
        );
        assert_eq!(
            inputs.as_str(),
            r#"
let _list1 = values.int_lists.remove(1);
let _list0 = values.int_lists.remove(0);
"#
            .trim_start_matches('\n')
        );
        let mut saved = Code::default();
        function.save(&mut saved, point);
        assert_eq!(
            saved.as_str(),
            r#"
values.ints.clear();
values.ints.extend_from_slice(&[b0_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[b0_v0]);
values.int_lists.clear();
values.int_lists.extend([b0_l0, b0_l1]);
"#
        );
    }

    #[test]
    fn list_edges_move_single_uses_clone_additional_uses_and_drop_omitted_columns() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
fn walk(left: List(Int), right: List(Int), steps: Int) -> Int {
  case steps {
    0 -> { let assert [first, ..] = left
           let assert [second, ..] = right
           first - second }
    _ -> walk(left, left, steps - 1)
  }
}
pub fn main() { walk([7], [2], 1) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let entry = shape.graph.entry();
        assert_eq!(entry, BlockId(0));
        let prefix = shape.checkpoints[shape.start(BlockId(0))];
        assert_eq!((prefix.ints, prefix.bools, prefix.int_lists), (1, 0, 2));
        let function = FunctionCodegen {
            name: "walk".into(),
            shape,
        };
        for (lists, step, expected_inputs, expected_code) in [
            (
                [0, 0],
                TransferStep {
                    source: 0,
                    destination: 1,
                },
                "_next",
                "let _next = (b0_i0, b0_l0.clone(), b0_l0,);\ndrop(b0_l1);\n",
            ),
            (
                [1, 0],
                TransferStep {
                    source: 1,
                    destination: 0,
                },
                "(b0_i0, b0_l1, b0_l0,)",
                "",
            ),
        ] {
            let edge = Edge {
                target: entry,
                args: function
                    .shape
                    .graph
                    .block(entry)
                    .params()
                    .iter()
                    .map(|slot| match slot.local() {
                        ParamLocal::List(ListLocal::Int { local, type_id }) => {
                            ParamLocal::List(ListLocal::Int {
                                local: IntListLocalId(lists[local.0]),
                                type_id: *type_id,
                            })
                        }
                        local => local.clone(),
                    })
                    .collect(),
                transfer: Transfer {
                    families: vec![FamilyTransfer {
                        family: StorageFamily::IntList,
                        length: 2,
                        steps: vec![step].into(),
                    }]
                    .into(),
                },
            };
            let mut source = Code::default();
            assert_eq!(
                function.edge_inputs(&mut source, entry, super::CompiledEdge::Ordinary(&edge)),
                expected_inputs
            );
            assert_eq!(source.as_str(), expected_code);
        }
    }

    #[test]
    fn whole_list_alias_edges_move_the_subject_beside_the_selected_tail() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
fn same(values: List(Int)) {
  let assert [_, ..tail] as original = values
  tail == original
}
pub fn main() { same([7]) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let function = FunctionCodegen {
            name: "same".into(),
            shape: CompiledShape::inspect(plan.bool_function(BoolFunctionId(1)).body()).unwrap(),
        };
        let matches = function
            .shape
            .blocks
            .iter()
            .filter_map(|(block, view)| match &view.terminator {
                CompiledTerminator::Match(view) => Some((BlockId(*block), view)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
        let (block, view) = matches[0];
        assert_eq!(block, BlockId(0));
        let mut source = Code::default();
        assert_eq!(
            function.edge_inputs(
                &mut source,
                block,
                super::CompiledEdge::Match(&view.matcher.success),
            ),
            "(m0, b0_l0,)"
        );
        assert_eq!(source.as_str(), "");
    }

    #[test]
    fn scalar_arithmetic_and_boolean_comparisons_have_exact_rust_expressions() {
        let block = BlockId(3);
        let left = IntegerOperand::Local(IntLocalId(2));
        let right = IntegerOperand::Immediate(-7);
        for (operation, expected) in [
            (NumericOperation::Add, "b3_i2 + -7_i128"),
            (NumericOperation::Subtract, "b3_i2 - -7_i128"),
            (NumericOperation::Multiply, "b3_i2 * -7_i128"),
            (
                NumericOperation::Divide,
                "if -7_i128 == 0 { 0_i128 } else { b3_i2 / -7_i128 }",
            ),
            (
                NumericOperation::Remainder,
                "if -7_i128 == 0 { 0_i128 } else { b3_i2 % -7_i128 }",
            ),
        ] {
            assert_eq!(
                int_expression(block, &NumericInteger::Binary(operation, left, right)),
                expected
            );
        }
        for operation in [NumericOperation::Divide, NumericOperation::Remainder] {
            assert_eq!(
                int_expression(
                    block,
                    &NumericInteger::Binary(operation, left, IntegerOperand::Immediate(0))
                ),
                "0_i128"
            );
        }
        let literal = IntegerLiteral::from(BigInt::from(i64::MIN));
        assert_eq!(
            int_expression(block, &NumericInteger::Value(&literal)),
            "-9223372036854775808_i128"
        );
        assert_eq!(
            int_expression(block, &NumericInteger::Negate(IntLocalId(4))),
            "-b3_i4"
        );
        for (comparison, expected) in [
            (NumericComparison::Equal, "b3_i2 == -7_i128"),
            (NumericComparison::NotEqual, "b3_i2 != -7_i128"),
            (NumericComparison::Less, "b3_i2 < -7_i128"),
            (NumericComparison::LessEqual, "b3_i2 <= -7_i128"),
            (NumericComparison::Greater, "b3_i2 > -7_i128"),
            (NumericComparison::GreaterEqual, "b3_i2 >= -7_i128"),
        ] {
            assert_eq!(
                test_expression(block, &CompiledTest::Compare(comparison, left, right)),
                expected
            );
        }
        assert_eq!(
            test_expression(block, &CompiledTest::Not(BoolLocalId(5))),
            "!b3_v5"
        );
    }

    #[test]
    fn integer_list_boolean_tests_route_to_exact_rust_expressions() {
        let block = BlockId(3);
        for (test, expected) in [
            (
                IntListTest::Length {
                    list: IntListLocalId(2),
                    length: 4,
                    at_least: true,
                },
                "b3_l2.len() >= 4",
            ),
            (
                IntListTest::Equal {
                    left: IntListLocalId(2),
                    right: IntListLocalId(5),
                    negate: true,
                },
                "!_lists.equal(&b3_l2, &b3_l5)",
            ),
        ] {
            assert_eq!(
                test_expression(block, &CompiledTest::IntList(test)),
                expected
            );
        }
    }

    #[test]
    fn list_match_and_source_failure_paths_have_exact_structured_code() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
fn head(values: List(Int), value: Int) {
  let assert [first, ..] = values
  first + value
}
pub fn main() { head([7], 3) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let function = FunctionCodegen {
            name: "head".into(),
            shape: CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap(),
        };
        let mut source = Code::default();
        function.path(&mut source, BlockId(0), None, ProgressOutput::Direct);
        assert_eq!(
            source.as_str(),
            r#"
if *budget == 0 {

    values.ints.clear();
    values.ints.extend_from_slice(&[b0_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    values.int_lists.clear();
    values.int_lists.extend([b0_l0]);
    return data::compiled::CompiledProgress::Yield(0);
}
let _matched = if !b0_l0.is_empty() {
    'pattern: {
        let mut _reader = _lists.prefix(&b0_l0, 1);
        let Some(_head0) = _reader.next() else {
            break 'pattern Ok(None);
        };
        let Some(_small0) = _head0.small() else {
            break 'pattern Err(());
        };
        let m0 = _small0;
        break 'pattern Ok(Some((m0,)));
    }
} else {
    Ok(None)
};
match _matched {
    Ok(Some((m0,))) => {
        *budget -= 1;
        let _next = (b0_i0, m0,);
        drop(b0_l0);
        let (b1_i0, b1_i1,) = _next;
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            values.int_lists.clear();
            values.int_lists.extend([]);
            return data::compiled::CompiledProgress::Yield(1);
        }
        *budget -= 1;
        let b1_i2 = b1_i1 + b1_i0;
        if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

            values.ints.clear();
            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            values.int_lists.clear();
            values.int_lists.extend([]);
            return data::compiled::CompiledProgress::Interpreted(2);
        }
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            values.int_lists.clear();
            values.int_lists.extend([]);
            return data::compiled::CompiledProgress::Yield(2);
        }
        *budget -= 1;

        values.ints.clear();
        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        values.int_lists.clear();
        values.int_lists.extend([]);
        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
    },
    Ok(None) => {
        *budget -= 1;
        let (b2_l0,) = (b0_l0,);
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            values.int_lists.clear();
            values.int_lists.extend([b2_l0]);
            return data::compiled::CompiledProgress::Yield(3);
        }

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        values.int_lists.clear();
        values.int_lists.extend([b2_l0]);
        data::compiled::CompiledProgress::Interpreted(3)
    },
    Err(()) => {

        values.ints.clear();
        values.ints.extend_from_slice(&[b0_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        values.int_lists.clear();
        values.int_lists.extend([b0_l0]);
        data::compiled::CompiledProgress::Interpreted(0)
    }
}
"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn resumed_integer_switch_saves_each_selected_branch_before_dispatch() {
        let source = r#"
fn choose(value: Int) {
  case value {
    0 -> value + 1
    1 -> value + 2
    _ -> value + 3
  }
}

pub fn main() { choose(0) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let function = FunctionCodegen {
            name: "choose".into(),
            shape: CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap(),
        };
        let point = function.shape.checkpoints[function.entry()];
        let mut output = Code::default();
        function.resume_terminator(
            &mut output,
            point,
            &function.shape.block(point.block).terminator,
        );
        assert_eq!(
            output.as_str(),
            r#"
if b0_i0 == 0_i128 {
    let (b1_i0,) = (b0_i0,);

    values.ints.clear();
    values.ints.extend_from_slice(&[b1_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    CompiledResume::Next(1)
} else if b0_i0 == 1_i128 {
    let (b2_i0,) = (b0_i0,);

    values.ints.clear();
    values.ints.extend_from_slice(&[b2_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    CompiledResume::Next(3)
} else {
    let (b3_i0,) = (b0_i0,);

    values.ints.clear();
    values.ints.extend_from_slice(&[b3_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    CompiledResume::Next(5)
}
"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn empty_switch_arms_and_unretained_region_outputs_emit_only_their_actual_work() {
        let source = r#"
fn choose(value: Int) {
  case value {
    0 -> value + 1
    _ -> value - 1
  }
}

pub fn main() { choose(7) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let point = shape.checkpoints[shape.start(shape.graph.entry())];
        let mut function = FunctionCodegen {
            name: "selected".into(),
            shape,
        };
        let edge = Edge::new(
            BlockId(1),
            vec![ParamLocal::Int(IntLocalId(0))],
            Transfer {
                families: vec![FamilyTransfer {
                    family: StorageFamily::Int,
                    length: 1,
                    steps: vec![].into(),
                }]
                .into(),
            },
        );
        let terminator = CompiledTerminator::Switch {
            subject: IntLocalId(0),
            clauses: &[],
            fallback: &edge,
        };
        let mut output = Code::default();
        function.branch(
            &mut output,
            point,
            &terminator,
            false,
            ProgressOutput::Direct,
            &mut |source, _, _| source.push_str("fallback\n"),
        );
        assert_eq!(
            output.as_str(),
            r#"
{
    fallback
}
"#
            .trim_start_matches('\n')
        );

        output = Code::default();
        function.resume_terminator(&mut output, point, &terminator);
        assert_eq!(
            output.as_str(),
            r#"
{
    let (b1_i0,) = (b0_i0,);

    values.ints.clear();
    values.ints.extend_from_slice(&[b1_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    CompiledResume::Next(1)
}
"#
            .trim_start_matches('\n')
        );

        // Drive the same structured-path edge callback as normal emission;
        // a direct formatter closure is a different generic instantiation.
        function.shape.blocks.get_mut(&0).unwrap().terminator = CompiledTerminator::Switch {
            subject: IntLocalId(0),
            clauses: &[],
            fallback: &edge,
        };
        output = Code::default();
        function.path(
            &mut output,
            BlockId(0),
            Some(BlockId(1)),
            ProgressOutput::Direct,
        );
        assert_eq!(
            output.as_str(),
            r#"
if *budget == 0 {

    values.ints.clear();
    values.ints.extend_from_slice(&[b0_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    return data::compiled::CompiledProgress::Yield(0);
}
*budget -= 1;
{
    (b0_i0,)
}
"#
            .trim_start_matches('\n')
        );

        // Artifact graph admission permits arithmetic whose outputs are not
        // retained. It still performs its nodes and has no Big prefix to save.
        let region = ArithmeticRegion {
            inputs: vec![IntLocalId(0)].into(),
            nodes: vec![
                ArithmeticNode::Add(ArithmeticOperand::Input(0), ArithmeticOperand::Immediate(1)),
                ArithmeticNode::Negate(ArithmeticOperand::Value(0)),
            ]
            .into(),
            outputs: vec![].into(),
            native: true,
        };
        let instruction = super::CompiledInstruction::Region {
            region: &region,
            outputs: vec![],
        };
        output = Code::default();
        function.instruction(&mut output, point, &instruction, ProgressOutput::Direct);
        assert_eq!(
            output.as_str(),
            r#"
let _r0_n0 = b0_i0 + 1_i128;
let _r0_n1 = -_r0_n0;
"#
            .trim_start_matches('\n')
        );
        output = Code::default();
        function.big_exit(&mut output, point, &instruction, 0, ProgressOutput::Direct);
        assert_eq!(output.as_str(), "");

        // Scalar and retained region outputs share the exact next checkpoint.
        // A Small remainder cannot promote; an Add or a retained region can.
        let next = function.shape.start(BlockId(1)) + 1;
        let point = function.shape.checkpoints[function.shape.start(BlockId(1))];
        let expected = r#"
if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

    values.ints.clear();
    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    return data::compiled::CompiledProgress::Interpreted(2);
}
"#
        .trim_start_matches('\n');
        output = Code::default();
        function.big_exit(
            &mut output,
            point,
            &function.shape.block(BlockId(1)).instructions[0],
            next,
            ProgressOutput::Direct,
        );
        assert_eq!(output.as_str(), expected);
        output = Code::default();
        function.big_exit(
            &mut output,
            point,
            &super::CompiledInstruction::Integer(
                IntLocalId(1),
                NumericInteger::Binary(
                    NumericOperation::Remainder,
                    IntegerOperand::Local(IntLocalId(0)),
                    IntegerOperand::Immediate(3),
                ),
            ),
            next,
            ProgressOutput::Direct,
        );
        assert_eq!(output.as_str(), "");
        let region = ArithmeticRegion {
            outputs: vec![ArithmeticOutput {
                value: 1,
                slot: ParamSlot::new(
                    ParamLocal::Int(IntLocalId(1)),
                    function.shape.graph.block(BlockId(1)).params()[0].shape(),
                ),
            }]
            .into(),
            ..region
        };
        function.big_exit(
            &mut output,
            point,
            &super::CompiledInstruction::Region {
                region: &region,
                outputs: vec![IntLocalId(1)],
            },
            next,
            ProgressOutput::Direct,
        );
        assert_eq!(output.as_str(), expected);
    }

    #[test]
    fn structured_paths_emit_exact_branch_expressions_and_simultaneous_join_arguments() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
fn choose(value: Int, flag: Bool) {
  case flag {
    True -> value + 1
    False -> value - 1
  }
}

pub fn main() { choose(7, True) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let params = graph.block(graph.entry()).params();
        let edge = Edge::new(
            BlockId(1),
            vec![
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Bool(BoolLocalId(0)),
            ],
            Transfer {
                families: [StorageFamily::Int, StorageFamily::Bool]
                    .into_iter()
                    .map(|family| FamilyTransfer {
                        family,
                        length: 1,
                        steps: vec![].into(),
                    })
                    .collect::<Vec<_>>()
                    .into(),
            },
        );
        let switch = |values: &[i64]| {
            Terminator::IntSwitch(IntSwitch {
                subject: IntLocalId(0),
                clauses: values
                    .iter()
                    .map(|&value| (IntegerLiteral::from(BigInt::from(value)), edge.clone()))
                    .collect::<Vec<_>>()
                    .into(),
                fallback: edge.clone(),
            })
        };
        for (terminator, expected) in [
            (
                Terminator::Jump(Jump::new(edge.clone())),
                r#"
{
    (b0_i0, b0_v0,)
}
"#,
            ),
            (
                Terminator::BoolBranch(BoolBranch {
                    subject: BoolLocalId(0),
                    true_: edge.clone(),
                    false_: edge.clone(),
                }),
                r#"
if b0_v0 {
    (b0_i0, b0_v0,)
} else {
    (b0_i0, b0_v0,)
}
"#,
            ),
            (
                Terminator::TestBranch(TestBranch::new(
                    BoolTest::GtInt {
                        left: IntegerOperand::Local(IntLocalId(0)),
                        right: IntegerOperand::Immediate(0),
                    },
                    edge.clone(),
                    edge.clone(),
                )),
                r#"
if b0_i0 > 0_i128 {
    (b0_i0, b0_v0,)
} else {
    (b0_i0, b0_v0,)
}
"#,
            ),
            (
                switch(&[]),
                r#"
{
    (b0_i0, b0_v0,)
}
"#,
            ),
            (
                switch(&[0]),
                r#"
if b0_i0 == 0_i128 {
    (b0_i0, b0_v0,)
} else {
    (b0_i0, b0_v0,)
}
"#,
            ),
            (
                switch(&[0, 1]),
                r#"
if b0_i0 == 0_i128 {
    (b0_i0, b0_v0,)
} else if b0_i0 == 1_i128 {
    (b0_i0, b0_v0,)
} else {
    (b0_i0, b0_v0,)
}
"#,
            ),
        ] {
            let body: ExecutionIntFunctionBody<Infallible> =
                ExecutionIntFunctionBody::<Infallible>::from_parts(
                    ProfiledBlockGraph::from_parts(
                        BlockId(0),
                        vec![
                            ProfiledBlock::new(params.to_vec(), vec![], terminator),
                            ProfiledBlock::new(
                                params.to_vec(),
                                vec![],
                                Terminator::Exit(BlockGraphExitId(0)),
                            ),
                        ],
                    ),
                    vec![FunctionExit::Return(IntLocalId(0))].into(),
                );
            let function = FunctionCodegen {
                name: "selected".into(),
                shape: CompiledShape::inspect(&body).unwrap(),
            };
            let point = function.shape.checkpoints[function.entry()];
            let mut branch_output = Code::default();
            function.branch(
                &mut branch_output,
                point,
                &function.shape.block(BlockId(0)).terminator,
                false,
                ProgressOutput::Direct,
                &mut emit_edge_expression,
            );
            assert_eq!(branch_output.as_str(), expected.trim_start_matches('\n'));
            let mut output = Code::default();
            function.path(
                &mut output,
                BlockId(0),
                Some(BlockId(1)),
                ProgressOutput::Direct,
            );
            let branch = expected.trim_matches('\n');
            let expected = format!(
                r#"
if *budget == 0 {{

    values.ints.clear();
    values.ints.extend_from_slice(&[b0_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[b0_v0]);
    return data::compiled::CompiledProgress::Yield(0);
}}
*budget -= 1;
let (b1_i0, b1_v0,) = {branch};
(b1_i0, b1_v0,)
"#
            );
            assert_eq!(output.as_str(), expected.trim_start_matches('\n'));
        }
    }

    // Three blocks: read the Bool entry argument, produce the selected literal,
    // and return. The five checkpoints include the completed literal outputs.
    // These exact protocol oracles stay local to their emitting owner.
    const INT_EXPECTED: &str = r#"
{

    enum CompiledResume {
        Next(usize),
        Exit(data::compiled::CompiledProgress),
    }

    fn numeric_int_0(
        point: usize,
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> data::compiled::CompiledProgress {

        const RESUME: [
            fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
            5
        ] = [
            |values, budget| CompiledResume::Exit(numeric_int_0_entry((values.bools[0],), values, budget)),
            numeric_int_0_resume_1,
            numeric_int_0_resume_2,
            numeric_int_0_resume_3,
            numeric_int_0_resume_4,
        ];

        let mut point = point;
        loop {
            match RESUME[point](values, budget) {
                CompiledResume::Next(next) => point = next,
                CompiledResume::Exit(progress) => return progress,
            }
        }
    }

    fn numeric_int_0_entry(
        inputs: (bool,),
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> data::compiled::CompiledProgress {
        let (b0_v0,) = inputs;
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[b0_v0]);
            return data::compiled::CompiledProgress::Yield(0);
        }
        *budget -= 1;
        if b0_v0 {
            let () = ();
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                return data::compiled::CompiledProgress::Yield(1);
            }
            *budget -= 1;
            let b1_i0 = 1_i128;
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                return data::compiled::CompiledProgress::Yield(2);
            }
            *budget -= 1;

            values.ints.clear();
            values.ints.extend_from_slice(&[b1_i0]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
        } else {
            let () = ();
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                return data::compiled::CompiledProgress::Yield(3);
            }
            *budget -= 1;
            let b2_i0 = 2_i128;
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                return data::compiled::CompiledProgress::Yield(4);
            }
            *budget -= 1;

            values.ints.clear();
            values.ints.extend_from_slice(&[b2_i0]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
        }
    }

    fn numeric_int_0_resume_1(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let () = ();
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
        }
        *budget -= 1;
        let b1_i0 = 1_i128;

        values.ints.clear();
        values.ints.extend_from_slice(&[b1_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        CompiledResume::Next(2)
    }

    fn numeric_int_0_resume_2(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let (b1_i0,) = (values.ints[0],);
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[b1_i0]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
        }
        *budget -= 1;

        values.ints.clear();
        values.ints.extend_from_slice(&[b1_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
    }

    fn numeric_int_0_resume_3(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let () = ();
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
        }
        *budget -= 1;
        let b2_i0 = 2_i128;

        values.ints.clear();
        values.ints.extend_from_slice(&[b2_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        CompiledResume::Next(4)
    }

    fn numeric_int_0_resume_4(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let (b2_i0,) = (values.ints[0],);
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[b2_i0]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
        }
        *budget -= 1;

        values.ints.clear();
        values.ints.extend_from_slice(&[b2_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
    }
    data::compiled::CompiledFunctions {
        ints: data::Storage::Static(&[
            data::compiled::CompiledFunction {
                function: data::function::IntFunctionId(0),
                implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                    entry: 0,
                    checkpoints: data::Storage::Static(&[
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 0,
                            ints: 0,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(1),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(1),
                            instruction: 1,
                            ints: 1,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
                            instruction: 1,
                            ints: 1,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                    ]),
                    run: numeric_int_0,
                }),
            },
        ]),
        bools: data::Storage::Static(&[
        ]),
        customs: data::Storage::Static(&[
        ]),
        int_lists: data::Storage::Static(&[
        ]),
        callbacks: data::compiled::CompiledCallbacks::interpreted(),
    }
}
"#;
    const BOOL_EXPECTED: &str = r#"
{

    enum CompiledResume {
        Next(usize),
        Exit(data::compiled::CompiledProgress),
    }

    fn numeric_bool_0(
        point: usize,
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> data::compiled::CompiledProgress {

        const RESUME: [
            fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
            5
        ] = [
            |values, budget| CompiledResume::Exit(numeric_bool_0_entry((values.bools[0],), values, budget)),
            numeric_bool_0_resume_1,
            numeric_bool_0_resume_2,
            numeric_bool_0_resume_3,
            numeric_bool_0_resume_4,
        ];

        let mut point = point;
        loop {
            match RESUME[point](values, budget) {
                CompiledResume::Next(next) => point = next,
                CompiledResume::Exit(progress) => return progress,
            }
        }
    }

    fn numeric_bool_0_entry(
        inputs: (bool,),
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> data::compiled::CompiledProgress {
        let (b0_v0,) = inputs;
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[b0_v0]);
            return data::compiled::CompiledProgress::Yield(0);
        }
        *budget -= 1;
        if b0_v0 {
            let () = ();
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                return data::compiled::CompiledProgress::Yield(1);
            }
            *budget -= 1;
            let b1_v0 = false;
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                return data::compiled::CompiledProgress::Yield(2);
            }
            *budget -= 1;

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[b1_v0]);
            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
        } else {
            let () = ();
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                return data::compiled::CompiledProgress::Yield(3);
            }
            *budget -= 1;
            let b2_v0 = true;
            if *budget == 0 {

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);
                return data::compiled::CompiledProgress::Yield(4);
            }
            *budget -= 1;

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[b2_v0]);
            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
        }
    }

    fn numeric_bool_0_resume_1(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let () = ();
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
        }
        *budget -= 1;
        let b1_v0 = false;

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[b1_v0]);
        CompiledResume::Next(2)
    }

    fn numeric_bool_0_resume_2(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let (b1_v0,) = (values.bools[0],);
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[b1_v0]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
        }
        *budget -= 1;

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[b1_v0]);
        CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
    }

    fn numeric_bool_0_resume_3(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let () = ();
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
        }
        *budget -= 1;
        let b2_v0 = true;

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[b2_v0]);
        CompiledResume::Next(4)
    }

    fn numeric_bool_0_resume_4(
        values: &mut data::compiled::numeric::NumericValues,
        budget: &mut usize,
    ) -> CompiledResume {
        let (b2_v0,) = (values.bools[0],);
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[]);
            values.bools.clear();
            values.bools.extend_from_slice(&[b2_v0]);
            return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
        }
        *budget -= 1;

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[b2_v0]);
        CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
    }
    data::compiled::CompiledFunctions {
        ints: data::Storage::Static(&[
        ]),
        bools: data::Storage::Static(&[
            data::compiled::CompiledFunction {
                function: data::function::BoolFunctionId(0),
                implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                    entry: 0,
                    checkpoints: data::Storage::Static(&[
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 0,
                            ints: 0,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(1),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(1),
                            instruction: 1,
                            ints: 0,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
                            instruction: 1,
                            ints: 0,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                    ]),
                    run: numeric_bool_0,
                }),
            },
        ]),
        customs: data::Storage::Static(&[
        ]),
        int_lists: data::Storage::Static(&[
        ]),
        callbacks: data::compiled::CompiledCallbacks::interpreted(),
    }
}
"#;

    #[test]
    fn plain_and_hosted_int_and_bool_families_emit_exact_execution_code_and_links() {
        macro_rules! check_family {
            ($source:expr, $hosted_source:expr, $return:ty, $family:expr, $expected:expr) => {{
                let typed =
                    crate::compile_typed_module("example", "src/example.gleam", $source).unwrap();
                let (bindings, _) = ModuleBuilder::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(bool,), $return>::new("main"))
                    .unwrap();
                let prepared = bindings.prepare();
                assert_eq!(
                    Rust::expression(&CompiledCodegen::new(
                        &prepared.program.functions,
                        &prepared.program.common.custom_types
                    )),
                    $expected.trim_matches('\n')
                );

                let typed = crate::compile_typed_host_program(
                    "example",
                    "example",
                    [PackageSource::new(
                        "example",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "example",
                            "src/example.gleam",
                            $hosted_source,
                        )],
                    )],
                    HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
                )
                .unwrap();
                let execution = crate::HostedExecution::try_from_module_plan(
                    crate::plan_host_program(typed).unwrap(),
                )
                .unwrap();
                // The standalone main occupies family slot zero. Its numeric
                // callee has exactly the same body, now linked as slot one.
                let expected = $expected
                    .trim_matches('\n')
                    .replace(
                        &format!("numeric_{}_0", $family.0),
                        &format!("numeric_{}_1", $family.0),
                    )
                    .replace(
                        &format!("{}FunctionId(0)", $family.1),
                        &format!("{}FunctionId(1)", $family.1),
                    );
                assert_eq!(
                    Rust::expression(&CompiledCodegen::new(
                        &execution.execution.program.functions,
                        &execution.execution.program.common.custom_types
                    )),
                    expected
                );
            }};
        }
        check_family!(
            r#"
pub fn main(flag: Bool) {
  case flag {
    True -> 1
    False -> 2
  }
}
"#,
            r#"
fn choose(flag: Bool) {
  case flag {
    True -> 1
    False -> 2
  }
}

pub fn main() { choose(True) }
"#,
            BigInt,
            ("int", "Int"),
            INT_EXPECTED
        );
        check_family!(
            r#"
pub fn main(flag: Bool) {
  case flag {
    True -> False
    False -> True
  }
}
"#,
            r#"
fn choose(flag: Bool) {
  case flag {
    True -> False
    False -> True
  }
}

pub fn main() { choose(True) }
"#,
            bool,
            ("bool", "Bool"),
            BOOL_EXPECTED
        );
    }

    #[test]
    fn unsupported_plain_and_hosted_programs_emit_an_explicit_interpreted_sidecar() {
        let source = "pub fn main() { 42 }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("main"))
            .unwrap();
        let prepared = bindings.prepare();
        assert_eq!(
            Rust::expression(&CompiledCodegen::new(
                &prepared.program.functions,
                &prepared.program.common.custom_types
            )),
            "data::compiled::CompiledFunctions::interpreted()"
        );
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
        assert_eq!(
            Rust::expression(&CompiledCodegen::new(
                &execution.execution.program.functions,
                &execution.execution.program.common.custom_types
            )),
            "data::compiled::CompiledFunctions::interpreted()"
        );
    }
    #[test]
    fn connected_targets_keep_native_entries_and_non_leaf_callbacks_canonical() {
        use crate::host::HostProviderModule;
        use crate::plan::execution::function::{ExecutionFunctionEntry, ExecutionFunctionRef};
        let source = r#"
type Item { Item(Int) }
@external(erlang, "native", "increment")
fn native_int(value: Int) -> Int
@external(erlang, "native", "invert")
fn native_bool(value: Bool) -> Bool
fn fold(items: List(Item), total: Int, step: fn(Int, Item) -> Int) {
  case items { [] -> total [head, ..tail] -> fold(tail, step(total, head), step) }
}
fn any(items: List(Item), flag: Bool, step: fn(Bool, Item) -> Bool) {
  case items { [] -> flag [head, ..tail] -> any(tail, step(flag, head), step) }
}
fn add(total: Int, item: Item) { let Item(value) = item total + value }
fn include(flag: Bool, item: Item) { let Item(value) = item flag || value > 0 }
fn relay_int(total: Int, item: Item) { add(total, item) }
fn relay_bool(flag: Bool, item: Item) { include(flag, item) }
fn effectful(flag: Bool, item: Item) { echo item include(flag, item) || False }
fn effectful_int(total: Int, item: Item) { echo item total + 1 }
pub fn main() {
  let unused = any([], False, effectful)
  let unused_int = fold([], 0, effectful_int)
  let total = fold([Item(2), Item(3)], 0, relay_int)
  case any([Item(7)], False, relay_bool) && native_bool(False) {
    True -> total + native_int(0)
    False -> 0
  }
}
"#;
        let providers =
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "example", "example",
            )
            .unwrap()
            .with_function("native_int", |value: BigInt| value + 1)
            .unwrap()
            .with_function("native_bool", |value: bool| !value)
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
            providers,
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut echo).unwrap(),
            crate::Value::Int(6.into())
        );
        assert!(echo.is_empty());
        let program = &execution.execution.program;
        let emitted = Rust::expression(&CompiledCodegen::new(
            &program.functions,
            &program.common.custom_types,
        ));
        assert!(emitted.contains("CustomLoopImplementation"));
        for (index, function) in program
            .functions
            .value_returns
            .int_functions
            .iter()
            .enumerate()
        {
            if matches!(function.as_ref(), ExecutionFunctionRef::Host(_)) {
                assert!(!emitted.contains(&format!("fn callback_int_{index}_call(")));
            }
        }
        for (index, function) in program
            .functions
            .value_returns
            .bool_functions
            .iter()
            .enumerate()
        {
            if matches!(function.as_ref(), ExecutionFunctionRef::Host(_)) {
                assert!(!emitted.contains(&format!("fn callback_bool_{index}_call(")));
            }
        }
    }
    #[test]
    fn a_straight_return_dispatches_without_a_next_variant() {
        let source = "pub fn main(left: Int, right: Int, flag: Bool) { case flag { True -> left False -> right } }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, entry) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                "main",
            ))
            .unwrap();
        let module = bindings.seal();
        assert_eq!(
            module
                .call(
                    &entry,
                    (BigInt::from(7), BigInt::from(9), true),
                    &mut Vec::new()
                )
                .unwrap(),
            BigInt::from(7)
        );
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                "main",
            ))
            .unwrap();
        let prepared = bindings.prepare();
        let emitted = Rust::expression(&CompiledCodegen::new(
            &prepared.program.functions,
            &prepared.program.common.custom_types,
        ));
        assert_eq!(
            emitted.split("\n    fn ").next().unwrap(),
            "{\n\n    enum CompiledResume {\n        Exit(data::compiled::CompiledProgress),\n    }\n"
        );
        let shape = CompiledShape::inspect(
            prepared.program.functions.value_returns.int_functions[0].body(),
        )
        .unwrap();
        let function = FunctionCodegen {
            name: "main".into(),
            shape,
        };
        assert!(!function.resumes_next());
        let mut code = Code::default();
        function.write_code(&mut code, false);
        assert_eq!(
            code.as_str(),
            r#"
fn main(
    point: usize,
    values: &mut data::compiled::numeric::NumericValues,
    budget: &mut usize,
) -> data::compiled::CompiledProgress {

    const RESUME: [
        fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
        3
    ] = [
        |values, budget| CompiledResume::Exit(main_entry((values.ints[0], values.ints[1], values.bools[0],), values, budget)),
        main_resume_1,
        main_resume_2,
    ];
    let CompiledResume::Exit(progress) = RESUME[point](values, budget);
    progress
}

fn main_entry(
    inputs: (i128, i128, bool,),
    values: &mut data::compiled::numeric::NumericValues,
    budget: &mut usize,
) -> data::compiled::CompiledProgress {
    let (b0_i0, b0_i1, b0_v0,) = inputs;
    if *budget == 0 {

        values.ints.clear();
        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
        values.bools.clear();
        values.bools.extend_from_slice(&[b0_v0]);
        return data::compiled::CompiledProgress::Yield(0);
    }
    *budget -= 1;
    if b0_v0 {
        let (b1_i0,) = (b0_i0,);
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[b1_i0]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return data::compiled::CompiledProgress::Yield(1);
        }
        *budget -= 1;

        values.ints.clear();
        values.ints.extend_from_slice(&[b1_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
    } else {
        let (b2_i0,) = (b0_i1,);
        if *budget == 0 {

            values.ints.clear();
            values.ints.extend_from_slice(&[b2_i0]);
            values.bools.clear();
            values.bools.extend_from_slice(&[]);
            return data::compiled::CompiledProgress::Yield(2);
        }
        *budget -= 1;

        values.ints.clear();
        values.ints.extend_from_slice(&[b2_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
    }
}

fn main_resume_1(
    values: &mut data::compiled::numeric::NumericValues,
    budget: &mut usize,
) -> CompiledResume {
    let (b1_i0,) = (values.ints[0],);
    if *budget == 0 {

        values.ints.clear();
        values.ints.extend_from_slice(&[b1_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
    }
    *budget -= 1;

    values.ints.clear();
    values.ints.extend_from_slice(&[b1_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
}

fn main_resume_2(
    values: &mut data::compiled::numeric::NumericValues,
    budget: &mut usize,
) -> CompiledResume {
    let (b2_i0,) = (values.ints[0],);
    if *budget == 0 {

        values.ints.clear();
        values.ints.extend_from_slice(&[b2_i0]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
    }
    *budget -= 1;

    values.ints.clear();
    values.ints.extend_from_slice(&[b2_i0]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
}
"#
        );
    }

    #[test]
    fn custom_bit_results_and_integer_list_results_keep_their_ordinary_target_tables() {
        let source = r#"
type Outcome { Done(List(Int)) Bad }
fn create() { [7, -9] }
fn scan(bits: BitArray) {
  case bits { <<_:8>> -> Done(create()) _ -> Bad }
}
fn sum(items: List(Int)) {
  let assert [left, right] = items
  left + right
}
pub fn main() {
  let assert Done(items) = scan(<<7>>)
  sum(items)
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int((-2).into())
        );
        let emitted = Rust::expression(&CompiledCodegen::new(
            &plan.program.functions,
            &plan.program.common.custom_types,
        ));
        assert_eq!(
            emitted
                .rsplit("\n    data::compiled::CompiledFunctions ")
                .next()
                .unwrap(),
            r#"{
        ints: data::Storage::Static(&[
            data::compiled::CompiledFunction {
                function: data::function::IntFunctionId(1),
                implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                    entry: 0,
                    checkpoints: data::Storage::Static(&[
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 1,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(1),
                            instruction: 0,
                            ints: 2,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(1),
                            instruction: 1,
                            ints: 3,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 1,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                    ]),
                    run: int_list_int_1,
                }),
            },
        ]),
        bools: data::Storage::Static(&[
        ]),
        customs: data::Storage::Static(&[
            data::compiled::CompiledFunction {
                function: 0,
                implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                    entry: 0,
                    checkpoints: data::Storage::Static(&[
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 1,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(1),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                    ]),
                    run: bit_array_custom_0,
                }),
            },
        ]),
        int_lists: data::Storage::Static(&[
            data::compiled::CompiledFunction {
                function: data::function::IntListFunctionId {
                    index: 0,
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                },
                implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                    entry: 0,
                    checkpoints: data::Storage::Static(&[
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 0,
                            ints: 0,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 1,
                            ints: 1,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 2,
                            ints: 2,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                        data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 3,
                            ints: 2,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 1,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        },
                    ]),
                    run: int_list_int_list_0,
                }),
            },
        ]),
        callbacks: data::compiled::CompiledCallbacks::interpreted(),
    }
}"#
        );
    }
}
