use super::shape::KernelKind;
use super::{
    CallbackReturns, Code, FunctionCodegen, ProgressOutput, Rust, local_expression, tuple,
};
use crate::plan::execution::compiled::{CompiledCheckpoint, CompiledLoopFunction};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BoolInstruction, BoolLocalId, CustomInstruction, CustomListLocalId, CustomLocalId,
    IntInstruction, IntLocalId, ListInstruction, ListLocal, ParamLocal, ParamSlot,
    ProfiledInstruction, ProfiledInstructionKind, StorageFamily, TypedListInstruction,
};
use crate::plan::execution::type_::CustomListTypeId;
use std::collections::BTreeSet;

pub(super) enum CustomLoopInstruction<'graph> {
    Index {
        output: CustomLocalId,
        list: CustomListLocalId,
        index: usize,
    },
    Tail {
        output: CustomListLocalId,
        list: CustomListLocalId,
        type_id: CustomListTypeId,
        count: usize,
    },
    Call {
        function: CompiledLoopFunction,
        args: &'graph [ParamLocal],
        output: LoopCallOutput,
    },
}

pub(super) enum LoopCallOutput {
    Int(IntLocalId),
    Bool(BoolLocalId),
}

impl LoopCallOutput {
    pub(super) fn local(&self) -> ParamLocal {
        match self {
            Self::Int(local) => ParamLocal::Int(*local),
            Self::Bool(local) => ParamLocal::Bool(*local),
        }
    }
}

impl<'graph> CustomLoopInstruction<'graph> {
    pub(super) fn inspect<Graph: ExecutionGraphProfile>(
        instruction: &'graph ProfiledInstruction<Graph>,
        params: &[ParamSlot],
    ) -> Option<Self> {
        let ProfiledInstruction::Value(value) = instruction else {
            return None;
        };
        Some(match (value.kind(), value.output().local()) {
            (
                ProfiledInstructionKind::Custom(CustomInstruction::ListIndex { list, index }),
                ParamLocal::Custom(output),
            ) => Self::Index {
                output: output.id(),
                list: *list,
                index: *index,
            },
            (
                ProfiledInstructionKind::List(ListInstruction::Custom(
                    type_id,
                    TypedListInstruction::DropFirst { list, count },
                )),
                ParamLocal::List(ListLocal::Custom { local, .. }),
            ) => Self::Tail {
                output: *local,
                list: *list,
                type_id: *type_id,
                count: *count,
            },
            (
                ProfiledInstructionKind::Int(IntInstruction::FunctionCall {
                    function, args, ..
                }),
                ParamLocal::Int(output),
            ) => {
                let function = params.iter().find_map(|param| match param.local() {
                    ParamLocal::IntFunction { local, type_ } if local == function => {
                        Some(CompiledLoopFunction::Int {
                            local: *local,
                            type_: type_.clone(),
                        })
                    }
                    _ => None,
                })?;
                if !args.iter().all(|arg| {
                    matches!(
                        arg,
                        ParamLocal::Int(_) | ParamLocal::Bool(_) | ParamLocal::Custom(_)
                    )
                }) {
                    return None;
                }
                Self::Call {
                    function,
                    args,
                    output: LoopCallOutput::Int(*output),
                }
            }
            (
                ProfiledInstructionKind::Bool(BoolInstruction::FunctionCall {
                    function, args, ..
                }),
                ParamLocal::Bool(output),
            ) => {
                let function = params.iter().find_map(|param| match param.local() {
                    ParamLocal::BoolFunction { local, type_ } if local == function => {
                        Some(CompiledLoopFunction::Bool {
                            local: *local,
                            type_: type_.clone(),
                        })
                    }
                    _ => None,
                })?;
                if !args.iter().all(|arg| {
                    matches!(
                        arg,
                        ParamLocal::Int(_) | ParamLocal::Bool(_) | ParamLocal::Custom(_)
                    )
                }) {
                    return None;
                }
                Self::Call {
                    function,
                    args,
                    output: LoopCallOutput::Bool(*output),
                }
            }
            _ => return None,
        })
    }
}

impl<Graph: ExecutionGraphProfile> FunctionCodegen<'_, '_, Graph> {
    pub(super) fn loop_edge_inputs(
        &self,
        source: &mut Code,
        from: super::BlockId,
        inputs: Vec<(StorageFamily, String)>,
        point: CompiledCheckpoint,
    ) -> String {
        let mut transferred = Vec::new();
        let mut drops = Vec::new();
        for (family, prefix, count) in [
            (StorageFamily::Int, "i", point.ints),
            (StorageFamily::Bool, "v", point.bools),
            (StorageFamily::Custom, "c", point.customs),
            (StorageFamily::CustomList, "l", point.custom_lists),
            (StorageFamily::IntFunction, "f", point.int_functions),
            (StorageFamily::BoolFunction, "g", point.bool_functions),
        ] {
            let mut values = inputs
                .iter()
                .filter(|(kind, _)| *kind == family)
                .map(|(_, value)| value.clone())
                .collect::<Vec<_>>();
            if !matches!(family, StorageFamily::Int | StorageFamily::Bool) {
                let mut uses = BTreeSet::new();
                for value in values.iter_mut().rev() {
                    if !uses.insert(value.clone()) {
                        *value = format!("{value}.clone()");
                    }
                }
                for index in 0..count {
                    let value = format!("b{}_{prefix}{index}", from.0);
                    if !uses.contains(&value) {
                        drops.push(value);
                    }
                }
            }
            transferred.extend(values);
        }
        let inputs = tuple(transferred);
        if drops.is_empty() {
            return inputs;
        }
        source.push_str(&format!("let _next = {inputs};\n"));
        for value in drops {
            source.push_str(&format!("drop({value});\n"));
        }
        "_next".into()
    }

    pub(super) fn loop_preflight(
        &self,
        source: &mut Code,
        index: usize,
        instruction: &CustomLoopInstruction<'_>,
        output: ProgressOutput<'_>,
    ) {
        let CustomLoopInstruction::Index {
            output: local,
            list,
            index: element,
        } = instruction
        else {
            return;
        };
        let point = self.shape.checkpoints[index];
        source.open(&format!(
            "let b{}_c{} = match _lists.index(&b{}_l{}, {element}) {{\n",
            point.block.0, local.0, point.block.0, list.0
        ));
        source.push_str("Some(value) => value,\n");
        source.open("None => {\n");
        self.interpreted(source, index, true, output);
        source.close("}\n");
        source.close("};\n");
    }

    pub(super) fn loop_instruction(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        instruction: &CustomLoopInstruction<'_>,
        output: ProgressOutput<'_>,
    ) {
        match instruction {
            CustomLoopInstruction::Index { .. } => {}
            CustomLoopInstruction::Tail {
                output,
                list,
                type_id,
                count,
            } => source.push_str(&format!(
                "let b{}_l{} = _lists.tail(&b{}_l{}, {}, {count});\n",
                point.block.0,
                output.0,
                point.block.0,
                list.0,
                Rust::expression(type_id)
            )),
            CustomLoopInstruction::Call {
                function,
                args,
                output: local,
            } => {
                let (column, function_local) = match function {
                    CompiledLoopFunction::Int { local, .. } => ("f", local.0),
                    CompiledLoopFunction::Bool { local, .. } => ("g", local.0),
                };
                let args_for = |family| {
                    args.iter()
                        .filter_map(|arg| local_expression(point.block, arg))
                        .filter(|(kind, _)| *kind == family)
                        .map(|(_, value)| {
                            if family == StorageFamily::Custom {
                                format!("&{value}")
                            } else {
                                value
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                let result = match local {
                    LoopCallOutput::Int(local) => format!("b{}_i{}", point.block.0, local.0),
                    LoopCallOutput::Bool(local) => format!("b{}_v{}", point.block.0, local.0),
                };
                source.open(&format!(
                    "let {result} = match b{}_{column}{function_local}.call(\n",
                    point.block.0
                ));
                source.push_str(&format!("&data::compiled::custom_loop::CallbackArguments {{\n    ints: &[{}],\n    bools: &[{}],\n    customs: &[{}],\n}},\n&mut values.callee, budget,\n) {{\n", args_for(StorageFamily::Int), args_for(StorageFamily::Bool), args_for(StorageFamily::Custom)));
                source.push_str(
                    "data::compiled::custom_loop::CallbackProgress::Complete(value) => value,\n",
                );
                source.open(
                    "data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {\n",
                );
                self.save(source, point);
                let index = self.shape.start(point.block) + point.instruction;
                let progress = output.for_kind(format!("data::compiled::custom_loop::CustomLoopProgress::Call {{ point: {index}, progress }}"), KernelKind::CustomLoop);
                source.push_str(&format!("return {progress};\n"));
                source.close("}\n");
                source.close("};\n");
            }
        }
    }
}

impl<Graph: ExecutionGraphProfile> FunctionCodegen<'_, '_, Graph> {
    pub(super) fn write_callback(&self, source: &mut Code, returns: &CallbackReturns) {
        let function = self;
        let name = &function.name;
        let entry = function.shape.checkpoints[function.entry()];
        source.open(&format!("\nfn {name}_call(\n    inputs: &data::compiled::custom_loop::CallbackInputs<'_>,\n    values: &mut data::compiled::custom::CustomValues,\n    budget: &mut usize,\n) -> {} {{\n", returns.progress_type()));
        for index in 0..entry.ints {
            source.open(&format!(
                "let Some(_integer{index}) = inputs.integer({index}) else {{\n"
            ));
            source.push_str("return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);\n");
            source.close("};\n");
        }
        let inputs = tuple(
            (0..entry.ints)
                .map(|index| format!("_integer{index}"))
                .chain((0..entry.bools).map(|index| format!("inputs.boolean({index})")))
                .chain((0..entry.customs).map(|index| format!("inputs.custom({index})"))),
        );
        source.push_str(&format!("{name}_entry({inputs}, values, budget)\n"));
        source.close("}\n");
        function.signature(
            source,
            &format!("{name}_entry"),
            entry,
            returns.progress_type(),
        );
        source.push_str(&format!(
            "let {} = inputs;\n",
            function.locals(entry, false)
        ));
        function.path(source, entry.block, None, ProgressOutput::Callback(returns));
        source.close("}\n");
    }

    pub(super) fn write_callback_target(
        &self,
        source: &mut Code,
        function_id: &str,
        returns: &CallbackReturns,
    ) {
        let function = self;
        source.open("data::compiled::CompiledCallback {\n");
        source.push_str(&format!(
            "function: {},\nentry: {},\n",
            function_id,
            function.entry()
        ));
        source.open("checkpoints: data::Storage::Static(&[\n");
        for point in &function.shape.checkpoints {
            source.push_str(&format!("{},\n", Rust::expression(point)));
        }
        source.close("]),\n");
        source.open("returns: data::Storage::Static(&[\n");
        let locals = match returns {
            CallbackReturns::Int(locals) => locals.iter().map(Rust::expression).collect::<Vec<_>>(),
            CallbackReturns::Bool(locals) => {
                locals.iter().map(Rust::expression).collect::<Vec<_>>()
            }
        };
        for local in locals {
            source.push_str(&format!("{local},\n"));
        }
        source.close("]),\n");
        source.push_str(&format!("run: {}_call,\n", function.name));
        source.close("},\n");
    }
}

impl CallbackReturns {
    fn progress_type(&self) -> &'static str {
        match self {
            Self::Int(_) => "data::compiled::custom_loop::CallbackProgress<i128>",
            Self::Bool(_) => "data::compiled::custom_loop::CallbackProgress<bool>",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CustomLoopInstruction;
    use crate::plan::execution::graph::{
        BoolInstruction, IntInstruction, ProfiledInstruction, ProfiledInstructionKind,
    };
    use crate::{ExecutionPlan, Value, compile_typed_module, plan_module};

    #[test]
    fn loop_targets_preserve_checkpoints_calls_and_edge_ownership() {
        use super::{Code, FunctionCodegen, StorageFamily};
        use crate::plan::execution::prepared::codegen::shape::CompiledShape;
        let source = r#"
type Item { Item(Int) }
fn fold(items: List(Item), total: Int, step: fn(Int, Item) -> Int) {
  case items {
    [] -> total
    [head, ..tail] -> fold(tail, step(total, head), step)
  }
}
fn add(total: Int, item: Item) { let Item(value) = item total + value }
pub fn main() { fold([Item(2)], 0, add) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(2.into())
        );
        let program = &plan.program;
        let fold = &program.functions.value_returns.int_functions[2];
        let shape =
            CompiledShape::inspect_custom_loop(fold.body(), &program.common.custom_types).unwrap();
        let function = FunctionCodegen {
            name: "fold",
            shape: &shape,
        };
        let mut target = Code::default();
        function.write_target(&mut target, "data::function::IntFunctionId(2)");
        assert_eq!(
            target.as_str(),
            r#"data::compiled::CompiledFunction {
    function: data::function::IntFunctionId(2),
    implementation: data::compiled::CompiledImplementation::CustomLoop(data::Storage::Static(&data::compiled::CustomLoopImplementation {
        entry: 0,
        checkpoints: data::Storage::Static(&[
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(0),
                instruction: 0,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 1,
                int_functions: 1,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(1),
                instruction: 0,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(2),
                instruction: 0,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 1,
                int_functions: 1,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(2),
                instruction: 1,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 1,
                custom_lists: 1,
                int_functions: 1,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(2),
                instruction: 2,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 1,
                custom_lists: 2,
                int_functions: 1,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(2),
                instruction: 3,
                ints: 2,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 1,
                custom_lists: 2,
                int_functions: 1,
                bool_functions: 0,
            },
        ]),
        calls: data::Storage::Static(&[
            data::compiled::CompiledLoopCall {
                point: 4,
                function: data::compiled::CompiledLoopFunction::Int {
                    local: data::graph::IntFunctionLocalId(0),
                    type_: data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    },
                },
                args: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(1),
                        },
                    }),
                ]),
                output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
            },
        ]),
        run: fold,
    })),
},
"#
        );
        let point = function.shape.checkpoints[function.entry()];
        assert_eq!(
            (point.ints, point.custom_lists, point.int_functions),
            (1, 1, 1)
        );
        let mut code = Code::default();
        // The phase-local edge protocol duplicates each retained owner into
        // two destination slots. The source graph itself remains unchanged.
        let inputs = function.loop_edge_inputs(
            &mut code,
            point.block,
            vec![
                (StorageFamily::Int, "b0_i0".into()),
                (StorageFamily::CustomList, "b0_l0".into()),
                (StorageFamily::CustomList, "b0_l0".into()),
                (StorageFamily::IntFunction, "b0_f0".into()),
                (StorageFamily::IntFunction, "b0_f0".into()),
            ],
            point,
        );
        assert_eq!(code.as_str(), "");
        assert_eq!(
            inputs,
            "(b0_i0, b0_l0.clone(), b0_l0, b0_f0.clone(), b0_f0,)"
        );
        let inputs = function.loop_edge_inputs(
            &mut code,
            point.block,
            vec![(StorageFamily::Int, "b0_i0".into())],
            point,
        );
        assert_eq!(inputs, "_next");
        assert_eq!(
            code.as_str(),
            "let _next = (b0_i0,);\ndrop(b0_l0);\ndrop(b0_f0);\n"
        );
    }

    #[test]
    fn callback_calls_require_an_entry_function_and_supported_argument_families() {
        for (source, expected) in [
            (
                r#"
type Item { Item(Int) }
fn apply(item: Item) {
  let callback = fn(value: Item) { let Item(inner) = value inner }
  callback(item) + 0
}
pub fn main() { apply(Item(7)) }
"#,
                Value::Int(7.into()),
            ),
            (
                r#"
type Item { Item(Int) }
fn apply(item: Item) {
  let callback = fn(value: Item) { let Item(inner) = value inner > 0 }
  callback(item) || False
}
pub fn main() { apply(Item(7)) }
"#,
                Value::Bool(true),
            ),
            (
                r#"
type Item { Item(Int) }
fn apply(item: Item, callback: fn(String, Item) -> Int) { callback("text", item) + 0 }
fn read(_text: String, item: Item) { let Item(value) = item value }
pub fn main() { apply(Item(7), read) }
"#,
                Value::Int(7.into()),
            ),
            (
                r#"
type Item { Item(Int) }
fn apply(item: Item, callback: fn(String, Item) -> Bool) { callback("text", item) || False }
fn read(_text: String, item: Item) { let Item(value) = item value > 0 }
pub fn main() { apply(Item(7), read) }
"#,
                Value::Bool(true),
            ),
        ] {
            let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
            assert_eq!(crate::run_main(&plan, &mut Vec::new()).unwrap(), expected);
            let mut rejected = 0;
            for function in plan.program.functions.value_returns.int_functions.iter() {
                let graph = function.body().block_graph().as_view();
                let params = graph.block(graph.entry()).params();
                for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                    if matches!(instruction, ProfiledInstruction::Value(value) if matches!(value.kind(), ProfiledInstructionKind::Int(IntInstruction::FunctionCall { .. })))
                    {
                        assert!(CustomLoopInstruction::inspect(instruction, params).is_none());
                        rejected += 1;
                    }
                }
            }
            for function in plan.program.functions.value_returns.bool_functions.iter() {
                let graph = function.body().block_graph().as_view();
                let params = graph.block(graph.entry()).params();
                for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                    if matches!(instruction, ProfiledInstruction::Value(value) if matches!(value.kind(), ProfiledInstructionKind::Bool(BoolInstruction::FunctionCall { .. })))
                    {
                        assert!(CustomLoopInstruction::inspect(instruction, params).is_none());
                        rejected += 1;
                    }
                }
            }
            assert_eq!(rejected, 1, "{source}");
        }
    }
}
