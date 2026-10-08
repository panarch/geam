data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 25,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("native_loop"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/native_loop.gleam", r#"
@external(erlang, "native_loop", "observe")
fn observe(value: Int) -> Int

@external(erlang, "native_loop", "keep")
fn keep(value: a) -> a

@external(erlang, "native_loop", "begin")
fn begin() -> Nil

pub fn captured(count: Int, value: Int) -> Int {
  repeat(count, fn() { value })
}

pub fn computed(count: Int, value: Int) -> Int {
  repeat(count, fn() { value + 1 })
}

pub fn computed_cancellable(count: Int, value: Int) -> Int {
  begin()
  repeat(count, fn() { value + 1 })
}

pub fn cancellable(count: Int, value: Int) -> Int {
  begin()
  repeat(count, fn() { value })
}

fn repeat(count: Int, producer: fn() -> Int) -> Int {
  let result = observe(producer())
  case count {
    1 -> result
    _ -> repeat(count - 1, producer)
  }
}

pub fn retained_value(count: Int, value: Int) -> Int {
  repeat_opaque(count, fn() { value })
}

fn repeat_opaque(count: Int, producer: fn() -> Int) -> Int {
  let result = keep(producer())
  case count {
    1 -> result
    _ -> repeat_opaque(count - 1, producer)
  }
}

pub fn compound(value: List(Int)) -> List(Int) {
  repeat_compound(3, fn() { value })
}

fn repeat_compound(count: Int, producer: fn() -> List(Int)) -> List(Int) {
  let result = keep(producer())
  case count {
    1 -> result
    _ -> repeat_compound(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_float")
fn observe_float(value: Float) -> Float

pub fn captured_float(count: Int, value: Float) -> Float {
  repeat_float(count, fn() { value })
}

fn repeat_float(count: Int, producer: fn() -> Float) -> Float {
  let result = observe_float(producer())
  case count {
    1 -> result
    _ -> repeat_float(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_string")
fn observe_string(value: String) -> String

pub fn captured_string(count: Int, value: String) -> String {
  repeat_string(count, fn() { value })
}

fn repeat_string(count: Int, producer: fn() -> String) -> String {
  let result = observe_string(producer())
  case count {
    1 -> result
    _ -> repeat_string(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_bit_array")
fn observe_bit_array(value: BitArray) -> BitArray

pub fn captured_bit_array(count: Int, value: BitArray) -> BitArray {
  repeat_bit_array(count, fn() { value })
}

fn repeat_bit_array(count: Int, producer: fn() -> BitArray) -> BitArray {
  let result = observe_bit_array(producer())
  case count {
    1 -> result
    _ -> repeat_bit_array(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_utf_codepoint")
fn observe_utf_codepoint(value: UtfCodepoint) -> UtfCodepoint

pub fn captured_utf_codepoint(count: Int, value: UtfCodepoint) -> UtfCodepoint {
  repeat_utf_codepoint(count, fn() { value })
}

fn repeat_utf_codepoint(
  count: Int,
  producer: fn() -> UtfCodepoint,
) -> UtfCodepoint {
  let result = observe_utf_codepoint(producer())
  case count {
    1 -> result
    _ -> repeat_utf_codepoint(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_bool")
fn observe_bool(value: Bool) -> Bool

pub fn captured_bool(count: Int, value: Bool) -> Bool {
  repeat_bool(count, fn() { value })
}

pub fn computed_bool(count: Int, value: Bool) -> Bool {
  repeat_bool(count, fn() { !value })
}

fn repeat_bool(count: Int, producer: fn() -> Bool) -> Bool {
  let result = observe_bool(producer())
  case count {
    1 -> result
    _ -> repeat_bool(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_nil")
fn observe_nil(value: Nil) -> Nil

pub fn captured_nil(count: Int, value: Nil) -> Nil {
  repeat_nil(count, fn() { value })
}

fn repeat_nil(count: Int, producer: fn() -> Nil) -> Nil {
  let result = observe_nil(producer())
  case count {
    1 -> result
    _ -> repeat_nil(count - 1, producer)
  }
}

@external(erlang, "native_loop", "float_to_bool")
fn float_to_bool(value: Float) -> Bool

pub fn mixed(count: Int, value: Float) -> Bool {
  repeat_mixed(count, fn() { value })
}

fn repeat_mixed(count: Int, producer: fn() -> Float) -> Bool {
  let result = float_to_bool(producer())
  case count {
    1 -> result
    _ -> repeat_mixed(count - 1, producer)
  }
}

pub fn literal_nil(count: Int) -> Nil {
  repeat_nil(count, fn() { Nil })
}

pub fn computed_float(count: Int, value: Float) -> Float {
  repeat_float(count, fn() { value +. 1.0 })
}

pub fn retained_float(count: Int, value: Float) -> Float {
  repeat_generic(count, fn() { value })
}

pub fn retained_string(count: Int, value: String) -> String {
  repeat_generic(count, fn() { value })
}

pub fn retained_bit_array(count: Int, value: BitArray) -> BitArray {
  repeat_generic(count, fn() { value })
}

fn repeat_generic(count: Int, producer: fn() -> a) -> a {
  let result = keep(producer())
  case count {
    1 -> result
    _ -> repeat_generic(count - 1, producer)
  }
}

pub fn graph_captured(count: Int, value: Int) -> Int {
  repeat_graph(count, fn() { value })
}

fn graph_keep(value: Int) -> Int {
  value
}

fn repeat_graph(count: Int, producer: fn() -> Int) -> Int {
  let result = graph_keep(producer())
  case count {
    1 -> result
    _ -> repeat_graph(count - 1, producer)
  }
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(0))),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[]),
                    int_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(6)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(0),
                                                            source: data::graph::IntLocalId(1),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(7),
                                            site: data::source::HostCallSite::from_static("native_loop", "captured", data::source::SourceSpan::new(252, 281)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(8)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(0),
                                                            source: data::graph::IntLocalId(1),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(7),
                                            site: data::source::HostCallSite::from_static("native_loop", "computed", data::source::SourceSpan::new(336, 369)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::Call {
                                                function: data::function::NilFunctionId(2),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "computed_cancellable", data::source::SourceSpan::new(436, 443)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(9)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(0),
                                                            source: data::graph::IntLocalId(1),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(7),
                                            site: data::source::HostCallSite::from_static("native_loop", "computed_cancellable", data::source::SourceSpan::new(446, 479)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::Call {
                                                function: data::function::NilFunctionId(2),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "cancellable", data::source::SourceSpan::new(537, 544)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(10)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(0),
                                                            source: data::graph::IntLocalId(1),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(7),
                                            site: data::source::HostCallSite::from_static("native_loop", "cancellable", data::source::SourceSpan::new(547, 576)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(11)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(0),
                                                            source: data::graph::IntLocalId(1),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(12),
                                            site: data::source::HostCallSite::from_static("native_loop", "retained_value", data::source::SourceSpan::new(801, 837)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(13)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(0),
                                                            source: data::graph::IntLocalId(1),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(14),
                                            site: data::source::HostCallSite::from_static("native_loop", "graph_captured", data::source::SourceSpan::new(4878, 4913)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 2,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::IntFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(657, 667)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(15),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(649, 668)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 2,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::IntFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(922, 932)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(16),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(917, 933)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 2,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::IntFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5049, 5059)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(17),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5038, 5060)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                    float_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FloatFunction {
                                                    local: data::graph::FloatFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                                family: data::function::FunctionReturnFamily::Float,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Float(data::function::FloatFunctionId(3)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Float {
                                                            target: data::graph::FloatLocalId(0),
                                                            source: data::graph::FloatLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::FloatFunctionId(4),
                                            site: data::source::HostCallSite::from_static("native_loop", "captured_float", data::source::SourceSpan::new(1448, 1483)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Float,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FloatFunction {
                                                    local: data::graph::FloatFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                                family: data::function::FunctionReturnFamily::Float,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Float(data::function::FloatFunctionId(6)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Float {
                                                            target: data::graph::FloatLocalId(0),
                                                            source: data::graph::FloatLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::FloatFunctionId(4),
                                            site: data::source::HostCallSite::from_static("native_loop", "computed_float", data::source::SourceSpan::new(4283, 4325)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Float,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FloatFunction {
                                                    local: data::graph::FloatFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                                family: data::function::FunctionReturnFamily::Float,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Float(data::function::FloatFunctionId(7)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Float {
                                                            target: data::graph::FloatLocalId(0),
                                                            source: data::graph::FloatLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::FloatFunctionId(8),
                                            site: data::source::HostCallSite::from_static("native_loop", "retained_float", data::source::SourceSpan::new(4390, 4427)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Float,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::FloatLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Float(data::graph::FloatLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Float,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::FloatFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::FloatFunction {
                                                            local: data::graph::FloatFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Float,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::FloatFunction {
                                                            local: data::graph::FloatFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::FunctionCall {
                                                function: data::graph::FloatFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_float", data::source::SourceSpan::new(1580, 1590)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(1)),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::Call {
                                                function: data::function::FloatFunctionId(9),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_float", data::source::SourceSpan::new(1566, 1591)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::FloatLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::FloatLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(1)),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::Value(f64::from_bits(4607182418800017408))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(2)),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::Add {
                                                left: data::graph::FloatLocalId(0),
                                                right: data::graph::FloatLocalId(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::FloatLocalId(2)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::FloatLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Float(data::graph::FloatLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Float,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::FloatFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::FloatFunction {
                                                            local: data::graph::FloatFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Float,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::FloatFunction {
                                                            local: data::graph::FloatFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::FunctionCall {
                                                function: data::graph::FloatFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4726, 4736)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(1)),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::Call {
                                                function: data::function::FloatFunctionId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4721, 4737)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::FloatLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 4,
                            return_: data::graph::FloatLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 11,
                            return_: data::graph::FloatLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    string_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::String,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::String(data::function::StringFunctionId(2)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::String {
                                                            target: data::graph::StringLocalId(0),
                                                            source: data::graph::StringLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::StringFunctionId(3),
                                            site: data::source::HostCallSite::from_static("native_loop", "captured_string", data::source::SourceSpan::new(1832, 1868)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::String,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::String,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::String(data::function::StringFunctionId(4)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::String {
                                                            target: data::graph::StringLocalId(0),
                                                            source: data::graph::StringLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::StringFunctionId(5),
                                            site: data::source::HostCallSite::from_static("native_loop", "retained_string", data::source::SourceSpan::new(4495, 4532)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::String,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::String,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::StringFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_string", data::source::SourceSpan::new(1969, 1979)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_string", data::source::SourceSpan::new(1954, 1980)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::String,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::StringFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4726, 4736)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4721, 4737)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 5,
                            return_: data::graph::StringLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 12,
                            return_: data::graph::StringLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    bit_array_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArrayFunction {
                                                    local: data::graph::BitArrayFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                                family: data::function::FunctionReturnFamily::BitArray,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::BitArray(data::function::BitArrayFunctionId(2)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::BitArray {
                                                            target: data::graph::BitArrayLocalId(0),
                                                            source: data::graph::BitArrayLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BitArrayFunctionId(3),
                                            site: data::source::HostCallSite::from_static("native_loop", "captured_bit_array", data::source::SourceSpan::new(2239, 2278)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::BitArray,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArrayFunction {
                                                    local: data::graph::BitArrayFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                                family: data::function::FunctionReturnFamily::BitArray,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::BitArray(data::function::BitArrayFunctionId(4)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::BitArray {
                                                            target: data::graph::BitArrayLocalId(0),
                                                            source: data::graph::BitArrayLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BitArrayFunctionId(5),
                                            site: data::source::HostCallSite::from_static("native_loop", "retained_bit_array", data::source::SourceSpan::new(4607, 4644)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::BitArray,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BitArrayLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::BitArray,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::BitArrayFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::FunctionCall {
                                                function: data::graph::BitArrayFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_bit_array", data::source::SourceSpan::new(2389, 2399)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Call {
                                                function: data::function::BitArrayFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_bit_array", data::source::SourceSpan::new(2371, 2400)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BitArrayLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BitArrayLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::BitArray,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::BitArrayFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::FunctionCall {
                                                function: data::graph::BitArrayFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4726, 4736)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Call {
                                                function: data::function::BitArrayFunctionId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4721, 4737)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BitArrayLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 6,
                            return_: data::graph::BitArrayLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 13,
                            return_: data::graph::BitArrayLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    utf_codepoint_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                                            shape: data::type_::ValueShapeId(11),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::UtfCodepointFunction {
                                                    local: data::graph::UtfCodepointFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(12),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                                                },
                                                family: data::function::FunctionReturnFamily::UtfCodepoint,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::UtfCodepoint(data::function::UtfCodepointFunctionId(1)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::UtfCodepoint {
                                                            target: data::graph::UtfCodepointLocalId(0),
                                                            source: data::graph::UtfCodepointLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::UtfCodepointFunctionId(2),
                                            site: data::source::HostCallSite::from_static("native_loop", "captured_utf_codepoint", data::source::SourceSpan::new(2690, 2733)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::UtfCodepointFunction {
                                                local: data::graph::UtfCodepointFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::UtfCodepoint,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                                            shape: data::type_::ValueShapeId(11),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::UtfCodepointLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::UtfCodepoint,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::UtfCodepointFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::UtfCodepointFunction {
                                                            local: data::graph::UtfCodepointFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::UtfCodepoint,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::UtfCodepointFunction {
                                                            local: data::graph::UtfCodepointFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::UtfCodepointFunction {
                                                local: data::graph::UtfCodepointFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(12),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                                            shape: data::type_::ValueShapeId(11),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::UtfCodepointFunction {
                                                local: data::graph::UtfCodepointFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(12),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::UtfCodepoint(data::graph::UtfCodepointInstruction::FunctionCall {
                                                function: data::graph::UtfCodepointFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_utf_codepoint", data::source::SourceSpan::new(2867, 2877)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(1)),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::UtfCodepoint(data::graph::UtfCodepointInstruction::Call {
                                                function: data::function::UtfCodepointFunctionId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_utf_codepoint", data::source::SourceSpan::new(2845, 2878)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::UtfCodepointLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 7,
                            return_: data::graph::UtfCodepointLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    custom_functions: data::Storage::Static(&[]),
                    external_functions: data::Storage::Static(&[]),
                    bool_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(14),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(3)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Bool {
                                                            target: data::graph::BoolLocalId(0),
                                                            source: data::graph::BoolLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BoolFunctionId(4),
                                            site: data::source::HostCallSite::from_static("native_loop", "captured_bool", data::source::SourceSpan::new(3113, 3147)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Bool,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(14),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(5)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Bool {
                                                            target: data::graph::BoolLocalId(0),
                                                            source: data::graph::BoolLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BoolFunctionId(4),
                                            site: data::source::HostCallSite::from_static("native_loop", "computed_bool", data::source::SourceSpan::new(3209, 3244)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Bool,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FloatFunction {
                                                    local: data::graph::FloatFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                                family: data::function::FunctionReturnFamily::Float,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Float(data::function::FloatFunctionId(5)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Float {
                                                            target: data::graph::FloatLocalId(0),
                                                            source: data::graph::FloatLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BoolFunctionId(6),
                                            site: data::source::HostCallSite::from_static("native_loop", "mixed", data::source::SourceSpan::new(3921, 3956)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Float,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Bool,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::BoolFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(13),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                                function: data::graph::BoolFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3337, 3347)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(13),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3324, 3348)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(13),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Float,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::FloatFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::FloatFunction {
                                                            local: data::graph::FloatFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Float,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::FloatFunction {
                                                            local: data::graph::FloatFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FloatFunction {
                                                local: data::graph::FloatFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Float),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::FunctionCall {
                                                function: data::graph::FloatFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_mixed", data::source::SourceSpan::new(4052, 4062)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(13),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(8),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_mixed", data::source::SourceSpan::new(4038, 4063)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 8,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 10,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    nil_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::NilFunction {
                                                    local: data::graph::NilFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                },
                                                family: data::function::FunctionReturnFamily::Nil,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Nil(data::function::NilFunctionId(3)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Nil {
                                                            target: data::graph::NilLocalId(0),
                                                            source: data::graph::NilLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::NilFunctionId(4),
                                            site: data::source::HostCallSite::from_static("native_loop", "captured_nil", data::source::SourceSpan::new(3567, 3600)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::NilFunction {
                                                local: data::graph::NilFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::NilFunction {
                                                    local: data::graph::NilFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                },
                                                family: data::function::FunctionReturnFamily::Nil,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Nil(data::function::NilFunctionId(5)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::NilFunctionId(4),
                                            site: data::source::HostCallSite::from_static("native_loop", "literal_nil", data::source::SourceSpan::new(4187, 4218)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::NilFunction {
                                                local: data::graph::NilFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                },
                                            },
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::NilLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::NilLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Nil(data::graph::NilLocalId(1)),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::NilFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::NilFunction {
                                                            local: data::graph::NilFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::NilFunction {
                                                            local: data::graph::NilFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::NilFunction {
                                                local: data::graph::NilFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(15),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::NilFunction {
                                                local: data::graph::NilFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Nil),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(15),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::FunctionCall {
                                                function: data::graph::NilFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_nil", data::source::SourceSpan::new(3689, 3699)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::Call {
                                                function: data::function::NilFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_nil", data::source::SourceSpan::new(3677, 3700)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::NilLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..0,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::Value),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::NilLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 9,
                            return_: data::graph::NilLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    tuple_functions: data::Storage::Static(&[]),
                },
                list_returns: data::function::ListFunctionTables {
                    parameter_list_functions: data::Storage::Static(&[]),
                    int_list_functions: data::Storage::Static(&[
                        (data::function::IntListFunctionId {
                            index: 0,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    3,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                    local: data::graph::IntListFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::List,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::List(data::function::ListFunctionId::Int(data::function::IntListFunctionId {
                                                        index: 1,
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    })),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::IntList {
                                                            target: data::graph::IntListLocalId(0),
                                                            source: data::graph::IntListLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntListFunctionId {
                                                index: 2,
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            },
                                            site: data::source::HostCallSite::from_static("native_loop", "compound", data::source::SourceSpan::new(1067, 1101)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                local: data::graph::IntListFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                list_type: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::IntList,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                ]),
                            },
                        }))),
                        (data::function::IntListFunctionId {
                            index: 1,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntListLocalId(0)),
                                ]),
                            },
                        }))),
                        (data::function::IntListFunctionId {
                            index: 2,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                                local: data::graph::IntListLocalId(1),
                                                                type_id: data::type_::IntListTypeId {
                                                                    list_type: data::type_::ListTypeId(0),
                                                                },
                                                            }),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::IntList,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::IntListFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                            local: data::graph::IntListFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                            },
                                                            list_type: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                            local: data::graph::IntListFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                            },
                                                            list_type: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                local: data::graph::IntListFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                list_type: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                local: data::graph::IntListFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                list_type: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            }, data::graph::TypedListInstruction::FunctionCall {
                                                function: data::graph::ListFunctionLocal::Int {
                                                    local: data::graph::IntListFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_compound", data::source::SourceSpan::new(1200, 1210)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(1),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            }, data::graph::TypedListInstruction::Call {
                                                function: data::function::IntListFunctionId {
                                                    index: 3,
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("native_loop", "repeat_compound", data::source::SourceSpan::new(1195, 1211)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntListLocalId(0)),
                                ]),
                            },
                        }))),
                        (data::function::IntListFunctionId {
                            index: 3,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        }, data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 3,
                            return_: data::graph::IntListLocalId(0),
                            body: ::core::marker::PhantomData,
                        }))),
                    ]),
                    string_list_functions: data::Storage::Static(&[]),
                    bit_array_list_functions: data::Storage::Static(&[]),
                    utf_codepoint_list_functions: data::Storage::Static(&[]),
                    custom_list_functions: data::Storage::Static(&[]),
                    external_list_functions: data::Storage::Static(&[]),
                    float_list_functions: data::Storage::Static(&[]),
                    bool_list_functions: data::Storage::Static(&[]),
                    nil_list_functions: data::Storage::Static(&[]),
                    tuple_list_functions: data::Storage::Static(&[]),
                    parameter_list_list_functions: data::Storage::Static(&[]),
                    list_list_functions: data::Storage::Static(&[]),
                    function_list_functions: data::Storage::Static(&[]),
                },
                function_returns: data::function::FunctionFunctionTables {
                    int_function_functions: data::Storage::Static(&[]),
                    float_function_functions: data::Storage::Static(&[]),
                    string_function_functions: data::Storage::Static(&[]),
                    bit_array_function_functions: data::Storage::Static(&[]),
                    utf_codepoint_function_functions: data::Storage::Static(&[]),
                    custom_function_functions: data::Storage::Static(&[]),
                    external_function_functions: data::Storage::Static(&[]),
                    bool_function_functions: data::Storage::Static(&[]),
                    nil_function_functions: data::Storage::Static(&[]),
                    tuple_function_functions: data::Storage::Static(&[]),
                    generic_function_functions: data::Storage::Static(&[]),
                    never_function_functions: data::Storage::Static(&[]),
                    parameter_list_function_functions: data::Storage::Static(&[]),
                    parameter_list_list_function_functions: data::Storage::Static(&[]),
                    int_list_function_functions: data::Storage::Static(&[]),
                    string_list_function_functions: data::Storage::Static(&[]),
                    bit_array_list_function_functions: data::Storage::Static(&[]),
                    utf_codepoint_list_function_functions: data::Storage::Static(&[]),
                    custom_list_function_functions: data::Storage::Static(&[]),
                    external_list_function_functions: data::Storage::Static(&[]),
                    float_list_function_functions: data::Storage::Static(&[]),
                    bool_list_function_functions: data::Storage::Static(&[]),
                    nil_list_function_functions: data::Storage::Static(&[]),
                    tuple_list_function_functions: data::Storage::Static(&[]),
                    list_list_function_functions: data::Storage::Static(&[]),
                    function_list_function_functions: data::Storage::Static(&[]),
                    function_function_functions: data::Storage::Static(&[]),
                },
            },
            compiled: {

                enum CompiledResume {
                    Exit(data::compiled::CompiledProgress),
                }
                use data::compiled::calls::{BoolCallable, CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallNativeFailure, CallNativeInput, CallNativeOps, CallNativeReturn, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                enum FunctionState {
                    Int0Point0 { int0: i128, int1: i128 },
                    Int0Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int1Point0 { int0: i128, int1: i128 },
                    Int1Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int4Point0 { int0: i128, int1: i128 },
                    Int4Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int5Point0 { int0: i128, int1: i128 },
                    Int5Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int6Point0 { int0: i128 },
                    Int7Point0 { int0: i128, int_function0: IntCallable },
                    Int7Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int7Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    Int7Point3 { int0: i128 },
                    Int7Point4 { int0: i128, int_function0: IntCallable },
                    Int7Point5 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int8Point0 { int0: i128 },
                    Int8Point1 { int0: i128, int1: i128 },
                    Int9Point0 { int0: i128 },
                    Int9Point1 { int0: i128, int1: i128 },
                    Int10Point0 { int0: i128 },
                    Int11Point0 { int0: i128 },
                    Int12Point0 { int0: i128, int_function0: IntCallable },
                    Int12Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int12Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    Int12Point3 { int0: i128 },
                    Int12Point4 { int0: i128, int_function0: IntCallable },
                    Int12Point5 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int13Point0 { int0: i128 },
                    Int14Point0 { int0: i128, int_function0: IntCallable },
                    Int14Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int14Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    Int14Point3 { int0: i128 },
                    Int14Point4 { int0: i128, int_function0: IntCallable },
                    Int14Point5 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int17Point0 { int0: i128 },
                    Bool0Point0 { int0: i128, bool0: bool },
                    Bool0Point1 { int0: i128, bool0: bool, bool_function0: BoolCallable },
                    Bool1Point0 { int0: i128, bool0: bool },
                    Bool1Point1 { int0: i128, bool0: bool, bool_function0: BoolCallable },
                    Bool3Point0 { bool0: bool },
                    Bool4Point0 { int0: i128, bool_function0: BoolCallable },
                    Bool4Point1 { int0: i128, bool_function0: BoolCallable, bool0: bool },
                    Bool4Point2 { int0: i128, bool_function0: BoolCallable, bool0: bool, bool1: bool },
                    Bool4Point3 { bool0: bool },
                    Bool4Point4 { int0: i128, bool_function0: BoolCallable },
                    Bool4Point5 { int0: i128, bool_function0: BoolCallable, int1: i128 },
                    Bool5Point0 { bool0: bool },
                    Bool5Point1 { bool0: bool, bool1: bool },
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
                }
                impl FunctionState {
                    fn values(self) -> CallValues {
                        match self {
                            Self::Int0Point0 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int0Point1 { int0, int1, int_function0 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point0 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int1Point1 { int0, int1, int_function0 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int4Point0 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int4Point1 { int0, int1, int_function0 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int5Point0 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point1 { int0, int1, int_function0 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int6Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int7Point0 { int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int7Point1 { int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int7Point2 { int0, int_function0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int7Point3 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int7Point4 { int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int7Point5 { int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int8Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int8Point1 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int9Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int9Point1 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int10Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int11Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int12Point0 { int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int12Point1 { int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int12Point2 { int0, int_function0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int12Point3 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int12Point4 { int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int12Point5 { int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int13Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int14Point0 { int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int14Point1 { int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int14Point2 { int0, int_function0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int14Point3 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int14Point4 { int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int14Point5 { int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int17Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point0 { int0, bool0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point1 { int0, bool0, bool_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool1Point0 { int0, bool0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point1 { int0, bool0, bool_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool3Point0 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool4Point0 { int0, bool_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool4Point1 { int0, bool_function0, bool0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool4Point2 { int0, bool_function0, bool0, bool1 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool4Point3 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool4Point4 { int0, bool_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool4Point5 { int0, bool_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool5Point0 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool5Point1 { bool0, bool1 } => {
                                CallValues { ints: vec![], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Canonical { values, .. } => values,
                        }
                    }
                }
                enum IntReturn {
                    Int7Call0 { int0: i128, int_function0: IntCallable },
                    Int7Call1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int12Call0 { int0: i128, int_function0: IntCallable },
                    Int12Call1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int14Call0 { int0: i128, int_function0: IntCallable },
                    Int14Call1 { int0: i128, int_function0: IntCallable, int1: i128 },
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int7Call0 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(657, 667)),
                            Self::Int7Call1 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(649, 668)),
                            Self::Int12Call0 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(922, 932)),
                            Self::Int12Call1 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(917, 933)),
                            Self::Int14Call0 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5049, 5059)),
                            Self::Int14Call1 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5038, 5060)),
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        match self {
                            Self::Int7Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int7Point1 { int0, int_function0, int1 }
                            },
                            Self::Int7Call1 { int0, int_function0, int1 } => {
                                let int2 = result;
                                FunctionState::Int7Point2 { int0, int_function0, int1, int2 }
                            },
                            Self::Int12Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int12Point1 { int0, int_function0, int1 }
                            },
                            Self::Int12Call1 { int0, int_function0, int1 } => {
                                let int2 = result;
                                FunctionState::Int12Point2 { int0, int_function0, int1, int2 }
                            },
                            Self::Int14Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int14Point1 { int0, int_function0, int1 }
                            },
                            Self::Int14Call1 { int0, int_function0, int1 } => {
                                let int2 = result;
                                FunctionState::Int14Point2 { int0, int_function0, int1, int2 }
                            },
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                            Self::Int7Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int7Call1 { int0, int_function0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int12Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int12Call1 { int0, int_function0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int14Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int14Call1 { int0, int_function0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                        }
                    }
                }
                enum BoolReturn {
                    Bool4Call0 { int0: i128, bool_function0: BoolCallable },
                    Bool4Call1 { int0: i128, bool_function0: BoolCallable, bool0: bool },
                }
                impl BoolReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Bool4Call0 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3337, 3347)),
                            Self::Bool4Call1 { .. } => data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3324, 3348)),
                        }
                    }
                    fn small(self, result: bool) -> FunctionState {
                        match self {
                            Self::Bool4Call0 { int0, bool_function0 } => {
                                let bool0 = result;
                                FunctionState::Bool4Point1 { int0, bool_function0, bool0 }
                            },
                            Self::Bool4Call1 { int0, bool_function0, bool0 } => {
                                let bool1 = result;
                                FunctionState::Bool4Point2 { int0, bool_function0, bool0, bool1 }
                            },
                        }
                    }
                    fn resume(self, result: bool) -> FunctionState { self.small(result) }
                }
                enum IntFunctionReturn {
                }
                impl IntFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: IntCallable) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: IntCallable) -> FunctionState { self.small(result) }
                }
                enum BoolFunctionReturn {
                }
                impl BoolFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: BoolCallable) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: BoolCallable) -> FunctionState { self.small(result) }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Next(FunctionState),
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
                    IntCall { callee: FunctionState, caller: IntReturn },
                    IntTail { callee: FunctionState, completed: FunctionState, point: data::compiled::CompiledCheckpoint },
                    Int { value: i128, exit: data::graph::BlockGraphExitId },
                    IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                    IntScalarBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: IntReturn },
                    BoolCall { callee: FunctionState, caller: BoolReturn },
                    BoolTail { callee: FunctionState, completed: FunctionState, point: data::compiled::CompiledCheckpoint },
                    Bool { value: bool, exit: data::graph::BlockGraphExitId },
                    BoolBridge { function: data::function::BoolFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: BoolReturn },
                    BoolScalarBridge { function: data::function::BoolFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: BoolReturn },
                }
                struct FunctionExecution {
                    active: Option<FunctionActive>,
                    integer_returns: Vec<IntReturn>,
                    boolean_returns: Vec<BoolReturn>,
                    integer_function_returns: Vec<IntFunctionReturn>,
                    boolean_function_returns: Vec<BoolFunctionReturn>,
                }
                #[allow(clippy::large_enum_variant, reason = "The suspended caller stays in its existing execution allocation.")]
                enum FunctionActive {
                    Running(FunctionState),
                    IntCall { function: data::function::IntFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: IntReturn },
                    IntReturn { caller: IntReturn, returned: CallNativeReturn<CallInteger> },
                    BoolCall { function: data::function::BoolFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: BoolReturn },
                    BoolReturn { caller: BoolReturn, returned: CallNativeReturn<bool> },
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(FunctionActive::Running(active)),
                            integer_returns: Vec::new(),
                            boolean_returns: Vec::new(),
                            integer_function_returns: Vec::new(),
                            boolean_function_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(4)) => calls_int_4_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(5)) => calls_int_5_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(6)) => calls_int_6_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(7)) => calls_int_7_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(8)) => calls_int_8_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(9)) => calls_int_9_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(10)) => calls_int_10_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(11)) => calls_int_11_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(12)) => calls_int_12_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(13)) => calls_int_13_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(14)) => calls_int_14_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(17)) => calls_int_17_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)) => calls_bool_1_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(3)) => calls_bool_3_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(4)) => calls_bool_4_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(5)) => calls_bool_5_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(FunctionActive::Running(active));
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.integer_function_returns.capacity() * std::mem::size_of::<IntFunctionReturn>() + self.boolean_function_returns.capacity() * std::mem::size_of::<BoolFunctionReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
                        let mut active = match active {
                            FunctionActive::Running(active) => active,
                            FunctionActive::IntCall { function, site, input, caller } => {
                                return CallProgress::Int {
                                    function, site, arguments: input.arguments(),
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                };
                            },
                            FunctionActive::IntReturn { caller, returned } => {
                                if *budget == 0 {
                                    self.active = Some(FunctionActive::IntReturn { caller, returned });
                                    return CallProgress::Yield(self);
                                }
                                *budget -= 1;
                                caller.resume(returned.into_value())
                            },
                            FunctionActive::BoolCall { function, site, input, caller } => {
                                return CallProgress::Bool {
                                    function, site, arguments: input.arguments(),
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
                        loop {
                            match function_step(active, ops, budget) {
                                FunctionStep::Next(next) => active = next,
                                FunctionStep::Yield(active) => {
                                    self.active = Some(FunctionActive::Running(active));
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::IntCall { callee, caller } => {
                                    self.integer_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::IntTail { callee, completed, point } => {
                                    if self.integer_returns.is_empty() {
                                        return CallProgress::Interpreted { point, values: completed.values() };
                                    }
                                    *budget -= 1;
                                    active = callee;
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
                                FunctionStep::IntBridge { function, site, arguments, caller } => return CallProgress::Int {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                },
                                FunctionStep::IntScalarBridge { function, site, input, caller } => {
                                    return CallProgress::Int {
                                        function, site, arguments: input.arguments(),
                                        resume: Box::new(move |value| {
                                            self.active = Some(FunctionActive::Running(caller.resume(value)));
                                            self
                                        }),
                                    };
                                },
                                FunctionStep::BoolCall { callee, caller } => {
                                    self.boolean_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::BoolTail { callee, completed, point } => {
                                    if self.boolean_returns.is_empty() {
                                        return CallProgress::Interpreted { point, values: completed.values() };
                                    }
                                    *budget -= 1;
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
                                FunctionStep::BoolBridge { function, site, arguments, caller } => return CallProgress::Bool {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                },
                                FunctionStep::BoolScalarBridge { function, site, input, caller } => {
                                    return CallProgress::Bool {
                                        function, site, arguments: input.arguments(),
                                        resume: Box::new(move |value| {
                                            self.active = Some(FunctionActive::Running(caller.resume(value)));
                                            self
                                        }),
                                    };
                                },
                                FunctionStep::Canonical { target, point, values } => {
                                    match target {
                                        data::compiled::CallTarget::Int(function) => {
                                            if let Some(caller) = self.integer_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedInt {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                        self
                                                    }),
                                                };
                                            }
                                            return CallProgress::Interpreted { point, values };
                                        },
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
                                            return CallProgress::Interpreted { point, values };
                                        },
                                        data::compiled::CallTarget::IntFunction(function) => {
                                            if let Some(caller) = self.integer_function_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedIntFunction {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                        self
                                                    }),
                                                };
                                            }
                                            return CallProgress::Interpreted { point, values };
                                        },
                                        data::compiled::CallTarget::BoolFunction(function) => {
                                            if let Some(caller) = self.boolean_function_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedBoolFunction {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                        self
                                                    }),
                                                };
                                            }
                                            return CallProgress::Interpreted { point, values };
                                        },
                                    }
                                },
                            }
                        }
                    }
                    fn advance_native(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize, native: &mut CallNativeOps<'_>) -> Result<Option<CallProgress>, CallNativeFailure> {
                        let Some(active) = self.active.take() else { return Ok(Some(CallProgress::Yield(self))); };
                        let mut active = match active {
                            FunctionActive::Running(active) => active,
                            FunctionActive::IntCall { function, site, input, caller } => {
                                if let CallNativeOps::Int { function: target, native } = native && *target == function {
                                    if *budget == 0 {
                                        self.active = Some(FunctionActive::IntCall { function, site, input, caller });
                                        return Ok(Some(CallProgress::Yield(self)));
                                    }
                                    *budget -= 1;
                                    let Some(returned) = native.call(input, site)? else { return Ok(None); };
                                    if *budget == 0 {
                                        self.active = Some(FunctionActive::IntReturn { caller, returned });
                                        return Ok(Some(CallProgress::Yield(self)));
                                    }
                                    *budget -= 1;
                                    caller.resume(returned.into_value())
                                } else {
                                    return Ok(Some(CallProgress::Int {
                                        function, site, arguments: input.arguments(),
                                        resume: Box::new(move |value| {
                                            self.active = Some(FunctionActive::Running(caller.resume(value)));
                                            self
                                        }),
                                    }));
                                }
                            },
                            FunctionActive::IntReturn { caller, returned } => {
                                if *budget == 0 {
                                    self.active = Some(FunctionActive::IntReturn { caller, returned });
                                    return Ok(Some(CallProgress::Yield(self)));
                                }
                                *budget -= 1;
                                caller.resume(returned.into_value())
                            },
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
                                    return Ok(Some(CallProgress::Bool {
                                        function, site, arguments: input.arguments(),
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
                                FunctionStep::Next(next) => active = next,
                                FunctionStep::Yield(active) => {
                                    self.active = Some(FunctionActive::Running(active));
                                    return Ok(Some(CallProgress::Yield(self)));
                                },
                                FunctionStep::IntCall { callee, caller } => {
                                    self.integer_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::IntTail { callee, completed, point } => {
                                    if self.integer_returns.is_empty() {
                                        return Ok(Some(CallProgress::Interpreted { point, values: completed.values() }));
                                    }
                                    *budget -= 1;
                                    active = callee;
                                },
                                FunctionStep::Int { value, exit } => {
                                    if let Some(caller) = self.integer_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        self.integer_function_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return Ok(Some(CallProgress::Complete { exit, output: CallOutput::Int(value.into()), execution: self }));
                                    }
                                },
                                FunctionStep::IntBridge { function, site, arguments, caller } => return Ok(Some(CallProgress::Int {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                })),
                                FunctionStep::IntScalarBridge { function, site, input, caller } => {
                                    active = {
                                        if let CallNativeOps::Int { function: target, native } = native && *target == function {
                                            if *budget == 0 {
                                                self.active = Some(FunctionActive::IntCall { function, site, input, caller });
                                                return Ok(Some(CallProgress::Yield(self)));
                                            }
                                            *budget -= 1;
                                            let Some(returned) = native.call(input, site)? else { return Ok(None); };
                                            if *budget == 0 {
                                                self.active = Some(FunctionActive::IntReturn { caller, returned });
                                                return Ok(Some(CallProgress::Yield(self)));
                                            }
                                            *budget -= 1;
                                            caller.resume(returned.into_value())
                                        } else {
                                            return Ok(Some(CallProgress::Int {
                                                function, site, arguments: input.arguments(),
                                                resume: Box::new(move |value| {
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                    self
                                                }),
                                            }));
                                        }
                                    };
                                },
                                FunctionStep::BoolCall { callee, caller } => {
                                    self.boolean_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::BoolTail { callee, completed, point } => {
                                    if self.boolean_returns.is_empty() {
                                        return Ok(Some(CallProgress::Interpreted { point, values: completed.values() }));
                                    }
                                    *budget -= 1;
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
                                        return Ok(Some(CallProgress::Complete { exit, output: CallOutput::Bool(value), execution: self }));
                                    }
                                },
                                FunctionStep::BoolBridge { function, site, arguments, caller } => return Ok(Some(CallProgress::Bool {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                })),
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
                                            return Ok(Some(CallProgress::Bool {
                                                function, site, arguments: input.arguments(),
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
                                        data::compiled::CallTarget::Int(function) => {
                                            if let Some(caller) = self.integer_returns.pop() {
                                                let site = caller.site();
                                                return Ok(Some(CallProgress::InterpretedInt {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                        self
                                                    }),
                                                }));
                                            }
                                            return Ok(Some(CallProgress::Interpreted { point, values }));
                                        },
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
                                            return Ok(Some(CallProgress::Interpreted { point, values }));
                                        },
                                        data::compiled::CallTarget::IntFunction(function) => {
                                            if let Some(caller) = self.integer_function_returns.pop() {
                                                let site = caller.site();
                                                return Ok(Some(CallProgress::InterpretedIntFunction {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                        self
                                                    }),
                                                }));
                                            }
                                            return Ok(Some(CallProgress::Interpreted { point, values }));
                                        },
                                        data::compiled::CallTarget::BoolFunction(function) => {
                                            if let Some(caller) = self.boolean_function_returns.pop() {
                                                let site = caller.site();
                                                return Ok(Some(CallProgress::InterpretedBoolFunction {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                        self
                                                    }),
                                                }));
                                            }
                                            return Ok(Some(CallProgress::Interpreted { point, values }));
                                        },
                                    }
                                },
                            }
                        }
                    }
                }
                fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        FunctionState::Canonical { target, point, values } => FunctionStep::Canonical { target, point, values },
                        FunctionState::Int0Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(6), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(0), int1)]);
                            FunctionStep::Next(FunctionState::Int0Point1 { int0, int1, int_function0 })
                        },
                        FunctionState::Int0Point1 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1, int_function0 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int7Point0 { int0, int_function0: int_function0.clone() }, completed: FunctionState::Int0Point1 { int0, int1, int_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int1Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(8), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(0), int1)]);
                            FunctionStep::Next(FunctionState::Int1Point1 { int0, int1, int_function0 })
                        },
                        FunctionState::Int1Point1 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point1 { int0, int1, int_function0 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int7Point0 { int0, int_function0: int_function0.clone() }, completed: FunctionState::Int1Point1 { int0, int1, int_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int4Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(11), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(0), int1)]);
                            FunctionStep::Next(FunctionState::Int4Point1 { int0, int1, int_function0 })
                        },
                        FunctionState::Int4Point1 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point1 { int0, int1, int_function0 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int12Point0 { int0, int_function0: int_function0.clone() }, completed: FunctionState::Int4Point1 { int0, int1, int_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int5Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(13), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(0), int1)]);
                            FunctionStep::Next(FunctionState::Int5Point1 { int0, int1, int_function0 })
                        },
                        FunctionState::Int5Point1 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point1 { int0, int1, int_function0 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int14Point0 { int0, int_function0: int_function0.clone() }, completed: FunctionState::Int5Point1 { int0, int1, int_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int6Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int6Point0 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int7Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, ()) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int7Call0 { int0, int_function0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(657, 667)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int7Call0 { int0, int_function0 } }
                        },
                        FunctionState::Int7Point1 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point1 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntScalarBridge { function: data::function::IntFunctionId(15), site: data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(649, 668)), input: CallNativeInput::Int(int1.into()), caller: IntReturn::Int7Call1 { int0, int_function0, int1 } }
                        },
                        FunctionState::Int7Point2 { int0, int_function0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            match int0 {
                                1_i128 => FunctionStep::Next(FunctionState::Int7Point3 { int0: int2 }),
                                _ => FunctionStep::Next(FunctionState::Int7Point4 { int0, int_function0: int_function0.clone() }),
                            }
                        },
                        FunctionState::Int7Point3 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point3 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int7Point4 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point4 { int0, int_function0 }); }
                            *budget -= 1;
                            let int1 = int0 - 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(2),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int7Point5 { int0, int_function0, int1 })
                        },
                        FunctionState::Int7Point5 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point5 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int7Point0 { int0: int1, int_function0: int_function0.clone() })
                        },
                        FunctionState::Int8Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point1 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int8Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point1 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int9Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point1 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int9Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point1 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int10Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point0 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int11Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point0 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int12Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, ()) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int12Call0 { int0, int_function0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(922, 932)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int12Call0 { int0, int_function0 } }
                        },
                        FunctionState::Int12Point1 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point1 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntScalarBridge { function: data::function::IntFunctionId(16), site: data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(917, 933)), input: CallNativeInput::Int(int1.into()), caller: IntReturn::Int12Call1 { int0, int_function0, int1 } }
                        },
                        FunctionState::Int12Point2 { int0, int_function0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            match int0 {
                                1_i128 => FunctionStep::Next(FunctionState::Int12Point3 { int0: int2 }),
                                _ => FunctionStep::Next(FunctionState::Int12Point4 { int0, int_function0: int_function0.clone() }),
                            }
                        },
                        FunctionState::Int12Point3 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point3 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int12Point4 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point4 { int0, int_function0 }); }
                            *budget -= 1;
                            let int1 = int0 - 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(2),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int12Point5 { int0, int_function0, int1 })
                        },
                        FunctionState::Int12Point5 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point5 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int12Point0 { int0: int1, int_function0: int_function0.clone() })
                        },
                        FunctionState::Int13Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point0 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int14Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, ()) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int14Call0 { int0, int_function0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5049, 5059)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int14Call0 { int0, int_function0 } }
                        },
                        FunctionState::Int14Point1 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point1 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntCall { callee: FunctionState::Int17Point0 { int0: int1 }, caller: IntReturn::Int14Call1 { int0, int_function0, int1 } }
                        },
                        FunctionState::Int14Point2 { int0, int_function0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            match int0 {
                                1_i128 => FunctionStep::Next(FunctionState::Int14Point3 { int0: int2 }),
                                _ => FunctionStep::Next(FunctionState::Int14Point4 { int0, int_function0: int_function0.clone() }),
                            }
                        },
                        FunctionState::Int14Point3 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point3 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int14Point4 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point4 { int0, int_function0 }); }
                            *budget -= 1;
                            let int1 = int0 - 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(2),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int14Point5 { int0, int_function0, int1 })
                        },
                        FunctionState::Int14Point5 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point5 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int14Point0 { int0: int1, int_function0: int_function0.clone() })
                        },
                        FunctionState::Int17Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point0 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool0Point0 { int0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { int0, bool0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(3), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::bool(data::graph::BoolLocalId(0), bool0)]);
                            FunctionStep::Next(FunctionState::Bool0Point1 { int0, bool0, bool_function0 })
                        },
                        FunctionState::Bool0Point1 { int0, bool0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int0, bool0, bool_function0 }); }
                            FunctionStep::BoolTail { callee: FunctionState::Bool4Point0 { int0, bool_function0: bool_function0.clone() }, completed: FunctionState::Bool0Point1 { int0, bool0, bool_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 1,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 1,
                            } }
                        },
                        FunctionState::Bool1Point0 { int0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point0 { int0, bool0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(5), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::bool(data::graph::BoolLocalId(0), bool0)]);
                            FunctionStep::Next(FunctionState::Bool1Point1 { int0, bool0, bool_function0 })
                        },
                        FunctionState::Bool1Point1 { int0, bool0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point1 { int0, bool0, bool_function0 }); }
                            FunctionStep::BoolTail { callee: FunctionState::Bool4Point0 { int0, bool_function0: bool_function0.clone() }, completed: FunctionState::Bool1Point1 { int0, bool0, bool_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 1,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 1,
                            } }
                        },
                        FunctionState::Bool3Point0 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool3Point0 { bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool4Point0 { int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool4Point0 { int0, bool_function0 }); }
                            *budget -= 1;
                            let callable = &bool_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_1(target, &captures, ()) {
                                return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool4Call0 { int0, bool_function0 } };
                            }
                            FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3337, 3347)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: BoolReturn::Bool4Call0 { int0, bool_function0 } }
                        },
                        FunctionState::Bool4Point1 { int0, bool_function0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool4Point1 { int0, bool_function0, bool0 }); }
                            *budget -= 1;
                            FunctionStep::BoolScalarBridge { function: data::function::BoolFunctionId(7), site: data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3324, 3348)), input: CallNativeInput::Bool(bool0), caller: BoolReturn::Bool4Call1 { int0, bool_function0, bool0 } }
                        },
                        FunctionState::Bool4Point2 { int0, bool_function0, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool4Point2 { int0, bool_function0, bool0, bool1 }); }
                            *budget -= 1;
                            match int0 {
                                1_i128 => FunctionStep::Next(FunctionState::Bool4Point3 { bool0: bool1 }),
                                _ => FunctionStep::Next(FunctionState::Bool4Point4 { int0, bool_function0: bool_function0.clone() }),
                            }
                        },
                        FunctionState::Bool4Point3 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool4Point3 { bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool4Point4 { int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool4Point4 { int0, bool_function0 }); }
                            *budget -= 1;
                            let int1 = int0 - 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(2),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 1,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] } }; }
                            FunctionStep::Next(FunctionState::Bool4Point5 { int0, bool_function0, int1 })
                        },
                        FunctionState::Bool4Point5 { int0, bool_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool4Point5 { int0, bool_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool4Point0 { int0: int1, bool_function0: bool_function0.clone() })
                        },
                        FunctionState::Bool5Point0 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool5Point0 { bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool5Point1 { bool0, bool1 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool5Point1 { bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool5Point1 { bool0, bool1 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool1, exit: data::graph::BlockGraphExitId(0) }
                        },
                    }
                }
                fn calls_entry_0(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: ()) -> Option<FunctionState> {
                    let () = inputs;
                    match target.0 {
                        6 => Some(FunctionState::Int6Point0 { int0: captures.int(data::graph::IntLocalId(0))? }),
                        8 => Some(FunctionState::Int8Point0 { int0: captures.int(data::graph::IntLocalId(0))? }),
                        9 => Some(FunctionState::Int9Point0 { int0: captures.int(data::graph::IntLocalId(0))? }),
                        10 => Some(FunctionState::Int10Point0 { int0: captures.int(data::graph::IntLocalId(0))? }),
                        11 => Some(FunctionState::Int11Point0 { int0: captures.int(data::graph::IntLocalId(0))? }),
                        13 => Some(FunctionState::Int13Point0 { int0: captures.int(data::graph::IntLocalId(0))? }),
                        _ => None,
                    }
                }
                fn calls_entry_1(target: data::function::BoolFunctionId, captures: &CallCaptureInputs<'_>, inputs: ()) -> Option<FunctionState> {
                    let () = inputs;
                    match target.0 {
                        3 => Some(FunctionState::Bool3Point0 { bool0: captures.bool(data::graph::BoolLocalId(0))? }),
                        5 => Some(FunctionState::Bool5Point0 { bool0: captures.bool(data::graph::BoolLocalId(0))? }),
                        _ => None,
                    }
                }
                fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int0Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int0Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_int_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int1Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int1Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_int_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_4_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int4Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int4Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_4_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point, values) { return Some(execution); }
                    let active = calls_int_4_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_5_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int5Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int5Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_5_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point, values) { return Some(execution); }
                    let active = calls_int_5_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_6_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int6Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_6_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(6)), point, values) { return Some(execution); }
                    let active = calls_int_6_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_7_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(6) | data::function::IntFunctionId(8) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13)) { return None; }
                            FunctionState::Int7Point0 { int0: values.int(0)?, int_function0: values.int_function(0)? }
                        },
                        1 => FunctionState::Int7Point1 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int7Point2 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        3 => FunctionState::Int7Point3 { int0: values.int(0)? },
                        4 => FunctionState::Int7Point4 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        5 => FunctionState::Int7Point5 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_7_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point, values) { return Some(execution); }
                    let active = calls_int_7_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_8_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int8Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int8Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_8_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point, values) { return Some(execution); }
                    let active = calls_int_8_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_9_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int9Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int9Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_9_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point, values) { return Some(execution); }
                    let active = calls_int_9_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_10_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int10Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_10_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(10)), point, values) { return Some(execution); }
                    let active = calls_int_10_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_11_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int11Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_11_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point, values) { return Some(execution); }
                    let active = calls_int_11_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_12_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(6) | data::function::IntFunctionId(8) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13)) { return None; }
                            FunctionState::Int12Point0 { int0: values.int(0)?, int_function0: values.int_function(0)? }
                        },
                        1 => FunctionState::Int12Point1 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int12Point2 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        3 => FunctionState::Int12Point3 { int0: values.int(0)? },
                        4 => FunctionState::Int12Point4 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        5 => FunctionState::Int12Point5 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_12_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point, values) { return Some(execution); }
                    let active = calls_int_12_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_13_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int13Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_13_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point, values) { return Some(execution); }
                    let active = calls_int_13_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_14_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(6) | data::function::IntFunctionId(8) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13)) { return None; }
                            FunctionState::Int14Point0 { int0: values.int(0)?, int_function0: values.int_function(0)? }
                        },
                        1 => FunctionState::Int14Point1 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int14Point2 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        3 => FunctionState::Int14Point3 { int0: values.int(0)? },
                        4 => FunctionState::Int14Point4 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        5 => FunctionState::Int14Point5 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_14_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point, values) { return Some(execution); }
                    let active = calls_int_14_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_17_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int17Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_17_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point, values) { return Some(execution); }
                    let active = calls_int_17_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool0Point0 { int0: values.int(0)?, bool0: values.bool(0)? },
                        1 => FunctionState::Bool0Point1 { int0: values.int(0)?, bool0: values.bool(0)?, bool_function0: values.bool_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_bool_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool1Point0 { int0: values.int(0)?, bool0: values.bool(0)? },
                        1 => FunctionState::Bool1Point1 { int0: values.int(0)?, bool0: values.bool(0)?, bool_function0: values.bool_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_bool_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool3Point0 { bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_bool_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_4_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.bool_function_target(0)?, data::function::BoolFunctionId(3) | data::function::BoolFunctionId(5)) { return None; }
                            FunctionState::Bool4Point0 { int0: values.int(0)?, bool_function0: values.bool_function(0)? }
                        },
                        1 => FunctionState::Bool4Point1 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, bool0: values.bool(0)? },
                        2 => FunctionState::Bool4Point2 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, bool0: values.bool(0)?, bool1: values.bool(1)? },
                        3 => FunctionState::Bool4Point3 { bool0: values.bool(0)? },
                        4 => FunctionState::Bool4Point4 { int0: values.int(0)?, bool_function0: values.bool_function(0)? },
                        5 => FunctionState::Bool4Point5 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_4_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(4)), point, values) { return Some(execution); }
                    let active = calls_bool_4_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_5_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool5Point0 { bool0: values.bool(0)? },
                        1 => FunctionState::Bool5Point1 { bool0: values.bool(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_5_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(5)), point, values) { return Some(execution); }
                    let active = calls_bool_5_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }

                fn int_list_int_list_1(
                    point: usize,
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                        1
                    ] = [
                        |values, _lists, budget| {
                            let _list0 = values.int_lists.remove(0);
                            CompiledResume::Exit(int_list_int_list_1_entry((_list0,), values, _lists, budget))
                        },
                    ];
                    let CompiledResume::Exit(progress) = RESUME[point](values, _lists, budget);
                    progress
                }

                fn int_list_int_list_1_entry(
                    inputs: (data::compiled::int_list::IntList,),
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_l0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                }
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                    ]),
                    bools: data::Storage::Static(&[
                    ]),
                    customs: data::Storage::Static(&[
                    ]),
                    int_lists: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::IntListFunctionId {
                                index: 1,
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
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: int_list_int_list_1,
                            }),
                        },
                    ]),
                    callbacks: data::compiled::CompiledCallbacks::interpreted(),
                    native_loops: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(7)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(7)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(15)),
                                    producer: data::compiled::NativeLoopProducer::Int(data::graph::IntFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(649, 668)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(12)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(12)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(16)),
                                    producer: data::compiled::NativeLoopProducer::Int(data::graph::IntFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(917, 933)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(14)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(14)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(17)),
                                    producer: data::compiled::NativeLoopProducer::Int(data::graph::IntFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5038, 5060)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Float(data::function::FloatFunctionId(4)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Float(data::function::FloatFunctionId(4)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Float(data::function::FloatFunctionId(9)),
                                    producer: data::compiled::NativeLoopProducer::Float(data::graph::FloatFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_float", data::source::SourceSpan::new(1566, 1591)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Float(data::function::FloatFunctionId(8)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Float(data::function::FloatFunctionId(8)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Float(data::function::FloatFunctionId(10)),
                                    producer: data::compiled::NativeLoopProducer::Float(data::graph::FloatFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4721, 4737)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::String(data::function::StringFunctionId(3)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::String(data::function::StringFunctionId(3)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::String(data::function::StringFunctionId(6)),
                                    producer: data::compiled::NativeLoopProducer::String(data::graph::StringFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_string", data::source::SourceSpan::new(1954, 1980)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::String(data::function::StringFunctionId(5)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::String(data::function::StringFunctionId(5)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::String(data::function::StringFunctionId(7)),
                                    producer: data::compiled::NativeLoopProducer::String(data::graph::StringFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4721, 4737)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::BitArray(data::function::BitArrayFunctionId(3)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::BitArray(data::function::BitArrayFunctionId(3)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::BitArray(data::function::BitArrayFunctionId(6)),
                                    producer: data::compiled::NativeLoopProducer::BitArray(data::graph::BitArrayFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_bit_array", data::source::SourceSpan::new(2371, 2400)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::BitArray(data::function::BitArrayFunctionId(5)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::BitArray(data::function::BitArrayFunctionId(5)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::BitArray(data::function::BitArrayFunctionId(7)),
                                    producer: data::compiled::NativeLoopProducer::BitArray(data::graph::BitArrayFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_generic", data::source::SourceSpan::new(4721, 4737)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::UtfCodepoint(data::function::UtfCodepointFunctionId(2)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::UtfCodepoint(data::function::UtfCodepointFunctionId(2)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::UtfCodepoint(data::function::UtfCodepointFunctionId(3)),
                                    producer: data::compiled::NativeLoopProducer::UtfCodepoint(data::graph::UtfCodepointFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_utf_codepoint", data::source::SourceSpan::new(2845, 2878)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Bool(data::function::BoolFunctionId(4)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Bool(data::function::BoolFunctionId(4)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Bool(data::function::BoolFunctionId(7)),
                                    producer: data::compiled::NativeLoopProducer::Bool(data::graph::BoolFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3324, 3348)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 1,
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Bool(data::function::BoolFunctionId(6)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Bool(data::function::BoolFunctionId(6)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Bool(data::function::BoolFunctionId(8)),
                                    producer: data::compiled::NativeLoopProducer::Float(data::graph::FloatFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_mixed", data::source::SourceSpan::new(4038, 4063)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::NativeLoopTarget::Nil(data::function::NilFunctionId(4)),
                            implementation: data::compiled::CompiledImplementation::NativeLoop(data::Storage::Static(&data::compiled::NativeLoopImplementation {
                                function: data::compiled::NativeLoopTarget::Nil(data::function::NilFunctionId(4)),
                                entry: 0,
                                contract: data::compiled::NativeLoopContract {
                                    header: data::graph::BlockId(0),
                                    finished: data::graph::BlockId(1),
                                    repeat: data::graph::BlockId(2),
                                    exit: data::graph::BlockGraphExitId(0),
                                    native: data::compiled::NativeLoopTarget::Nil(data::function::NilFunctionId(6)),
                                    producer: data::compiled::NativeLoopProducer::Nil(data::graph::NilFunctionLocalId(0)),
                                    site: data::source::HostCallSite::from_static("native_loop", "repeat_nil", data::source::SourceSpan::new(3677, 3700)),
                                },
                                checkpoints: data::Storage::Static(&[data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }]),
                                run: data::compiled::native_loop::run,
                            })),
                        },
                    ]),
                    function_calls: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(6)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(0),
                                                source: data::graph::IntLocalId(1),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "captured", data::source::SourceSpan::new(252, 281)),
                                    },
                                ]),
                                start: calls_int_0_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(8)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(0),
                                                source: data::graph::IntLocalId(1),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "computed", data::source::SourceSpan::new(336, 369)),
                                    },
                                ]),
                                start: calls_int_1_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(11)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(0),
                                                source: data::graph::IntLocalId(1),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "retained_value", data::source::SourceSpan::new(801, 837)),
                                    },
                                ]),
                                start: calls_int_4_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(13)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(0),
                                                source: data::graph::IntLocalId(1),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "graph_captured", data::source::SourceSpan::new(4878, 4913)),
                                    },
                                ]),
                                start: calls_int_5_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(6)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_6_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
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
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
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
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(657, 667)),
                                    },
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(15))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat", data::source::SourceSpan::new(649, 668)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 3,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_7_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_8_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_9_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(10)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_10_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_11_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
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
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
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
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(922, 932)),
                                    },
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(16))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat_opaque", data::source::SourceSpan::new(917, 933)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 3,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_12_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_13_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
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
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
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
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5049, 5059)),
                                    },
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(17))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat_graph", data::source::SourceSpan::new(5038, 5060)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 3,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_14_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(17)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_17_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(3)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Bool {
                                                target: data::graph::BoolLocalId(0),
                                                source: data::graph::BoolLocalId(0),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(4)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "captured_bool", data::source::SourceSpan::new(3113, 3147)),
                                    },
                                ]),
                                start: calls_bool_0_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(5)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Bool {
                                                target: data::graph::BoolLocalId(0),
                                                source: data::graph::BoolLocalId(0),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(4)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "computed_bool", data::source::SourceSpan::new(3209, 3244)),
                                    },
                                ]),
                                start: calls_bool_1_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(3)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_3_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(4)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 1,
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
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        target: data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(0)),
                                        args: data::Storage::Static(&[]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3337, 3347)),
                                    },
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(7))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("native_loop", "repeat_bool", data::source::SourceSpan::new(3324, 3348)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 3,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_4_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(5)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_5_start,
                            })),
                        },
                    ]),
                }
            },
            constants: data::constant::ProfiledConstantTable {
                ints: data::Storage::Static(&[]),
                strings: data::Storage::Static(&[]),
                bit_arrays: data::Storage::Static(&[]),
                customs: data::Storage::Static(&[]),
                floats: data::Storage::Static(&[]),
                bools: data::Storage::Static(&[]),
                nils: data::Storage::Static(&[]),
                tuples: data::Storage::Static(&[]),
                parameter_lists: data::Storage::Static(&[]),
                parameter_list_lists: data::Storage::Static(&[]),
                int_lists: data::Storage::Static(&[]),
                string_lists: data::Storage::Static(&[]),
                bit_array_lists: data::Storage::Static(&[]),
                utf_codepoint_lists: data::Storage::Static(&[]),
                custom_lists: data::Storage::Static(&[]),
                external_lists: data::Storage::Static(&[]),
                float_lists: data::Storage::Static(&[]),
                bool_lists: data::Storage::Static(&[]),
                nil_lists: data::Storage::Static(&[]),
                tuple_lists: data::Storage::Static(&[]),
                list_lists: data::Storage::Static(&[]),
                function_lists: data::Storage::Static(&[]),
                functions: data::Storage::Static(&[]),
            },
            function_parameters: data::function::FunctionCatalog {
                families: [
                    0..0,
                    0..18,
                    18..29,
                    29..37,
                    37..45,
                    45..49,
                    0..0,
                    0..0,
                    49..58,
                    58..65,
                    0..0,
                    0..0,
                    65..69,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                ],
                functions: data::Storage::Static(&[
                    data::function::FunctionContract {
                        parameters: 0..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 8..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..12,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..14,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..14,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..14,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..14,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..16,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..16,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..18,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 18..19,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 19..20,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 20..21,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 21..23,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 23..25,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 25..27,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 27..27,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 27..29,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(6),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 29..29,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 29..29,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 29..29,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 29..31,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(6),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 31..32,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 32..33,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 33..35,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 35..37,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 37..37,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                shape: data::type_::ValueShapeId(7),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 37..39,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(8),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 39..39,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                shape: data::type_::ValueShapeId(7),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 39..41,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(8),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 41..42,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 42..43,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 43..45,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 45..47,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 47..47,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                shape: data::type_::ValueShapeId(9),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 47..49,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(10),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 49..49,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                shape: data::type_::ValueShapeId(9),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 49..51,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(10),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 51..52,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 52..53,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 53..55,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(11),
                        ]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 55..55,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                                shape: data::type_::ValueShapeId(11),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 55..57,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(12),
                        ]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 57..58,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(11),
                        ]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 58..60,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(13),
                        ]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 60..62,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(13),
                        ]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 62..64,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 64..64,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                shape: data::type_::ValueShapeId(13),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 64..66,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(14),
                        ]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 66..66,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                shape: data::type_::ValueShapeId(13),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 66..68,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(6),
                        ]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 68..69,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(13),
                        ]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 69..70,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(13),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 70..72,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 72..73,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 73..73,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 73..73,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 73..75,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(15),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 75..75,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 75..76,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 76..77,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 77..77,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                    local: data::graph::IntListLocalId(0),
                                    type_id: data::type_::IntListTypeId {
                                        list_type: data::type_::ListTypeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(3),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 77..79,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(4),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 79..80,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::FloatFunction {
                        local: data::graph::FloatFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Float),
                        },
                    },
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::FloatFunction {
                        local: data::graph::FloatFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Float),
                        },
                    },
                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::String),
                        },
                    },
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::String),
                        },
                    },
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::BitArrayFunction {
                        local: data::graph::BitArrayFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                        },
                    },
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::BitArrayFunction {
                        local: data::graph::BitArrayFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                        },
                    },
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::UtfCodepointFunction {
                        local: data::graph::UtfCodepointFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                        },
                    },
                    data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::BoolFunction {
                        local: data::graph::BoolFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                        },
                    },
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::FloatFunction {
                        local: data::graph::FloatFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Float),
                        },
                    },
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::NilFunction {
                        local: data::graph::NilFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Nil),
                        },
                    },
                    data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                        local: data::graph::IntListFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                        },
                        list_type: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[
                    data::type_::ListStorageTypeId::Int(data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    }),
                ]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
                lifetimes: data::Storage::Static(&[
                    data::host::HostValueLifetime::LoadedOwner,
                ]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[]),
                definitions: data::Storage::Static(&[]),
            },
            external_types: data::type_::ExternalTypeTable {
                types: data::Storage::Static(&[]),
                lifetimes: data::Storage::Static(&[]),
                definitions: data::Storage::Static(&[]),
            },
            value_shapes: data::type_::ValueShapeTable {
                shapes: data::Storage::Static(&[
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Nil,
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                    },
                    data::type_::ValueShapeDescriptor::Float,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(5),
                    },
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(7),
                    },
                    data::type_::ValueShapeDescriptor::BitArray,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(9),
                    },
                    data::type_::ValueShapeDescriptor::UtfCodepoint,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(11),
                    },
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(13),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(2),
                    },
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Nil,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                    }),
                    data::type_::ValueType::Float,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Float),
                    }),
                    data::type_::ValueType::String,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                    data::type_::ValueType::BitArray,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                    }),
                    data::type_::ValueType::UtfCodepoint,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
                    }),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Nil),
                    }),
                ]),
                custom_shapes: data::Storage::Static(&[]),
            },
        },
        entries: data::program::LibraryFunctionEntries {
            ints: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(1),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(2),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(3),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(4),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(5),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            floats: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::FloatFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::FloatFunctionId(1),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::FloatFunctionId(2),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            strings: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::StringFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::StringFunctionId(1),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            bit_arrays: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::BitArrayFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::BitArrayFunctionId(1),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            utf_codepoints: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::UtfCodepointFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            customs: data::Storage::Static(&[]),
            externals: data::Storage::Static(&[]),
            bools: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::BoolFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::BoolFunctionId(1),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::BoolFunctionId(2),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            nils: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::NilFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::NilFunctionId(1),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            tuples: data::Storage::Static(&[]),
            lists: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::LibraryListFunctionId::Int(data::function::IntListFunctionId {
                        index: 0,
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[
                                data::type_::IntListTypeId {
                                    list_type: data::type_::ListTypeId(0),
                                },
                            ]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            functions: data::Storage::Static(&[]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("captured"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("computed"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("computed_cancellable"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("cancellable"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 3,
            },
            data::Export {
                name: data::Text::Static("retained_value"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 4,
            },
            data::Export {
                name: data::Text::Static("graph_captured"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 5,
            },
            data::Export {
                name: data::Text::Static("compound"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("captured_float"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Float,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Float),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("captured_string"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("captured_bit_array"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::BitArray,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("captured_utf_codepoint"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::UtfCodepoint,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::UtfCodepoint),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("captured_bool"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Bool,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("computed_bool"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Bool,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("captured_nil"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Nil,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Nil),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("mixed"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Float,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("literal_nil"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Nil),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("computed_float"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Float,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Float),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("retained_float"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Float,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Float),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("retained_string"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("retained_bit_array"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::BitArray,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                },
                slot: 1,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "begin", data::source::SourceSpan::new(182, 192)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Nil),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Nil),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Nil,
                layout: data::Storage::Static(&[]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "observe", data::source::SourceSpan::new(44, 66)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Int(data::graph::IntLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Int),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Int,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Int(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "keep", data::source::SourceSpan::new(116, 133)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Int),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "keep", data::source::SourceSpan::new(116, 133)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int))),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    shape: data::type_::ValueShapeId(3),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    })),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)), data::type_::ListTypeId(0)),
                    ]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "observe_float", data::source::SourceSpan::new(1346, 1376)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Float,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Float),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Float(data::graph::FloatLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Float,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Float),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Float,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Float,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Float(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "observe_string", data::source::SourceSpan::new(1724, 1756)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::String(data::graph::StringLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::String,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::String),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::String,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::String,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::String(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "observe_bit_array", data::source::SourceSpan::new(2117, 2154)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::BitArray(data::graph::BitArrayLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::BitArray,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::BitArray,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::BitArray(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "observe_utf_codepoint", data::source::SourceSpan::new(2544, 2589)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::UtfCodepoint,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::UtfCodepoint),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::UtfCodepoint(data::graph::UtfCodepointLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::UtfCodepoint,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::UtfCodepoint),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::UtfCodepoint,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::UtfCodepoint,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::UtfCodepoint(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "observe_bool", data::source::SourceSpan::new(3017, 3045)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Bool(data::graph::BoolLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Bool,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Bool,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Bool(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "observe_nil", data::source::SourceSpan::new(3477, 3503)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Nil,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Nil),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Nil(data::graph::NilLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Nil,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Nil),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Nil,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Nil,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Nil(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "float_to_bool", data::source::SourceSpan::new(3830, 3860)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Float,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Float(data::graph::FloatLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Float,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Float,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Bool,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Float(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "keep", data::source::SourceSpan::new(116, 133)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Float,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Float),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Float,
                    shape: data::type_::ValueShapeId(5),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Float(data::graph::FloatLocalId(0))),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Float,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Float),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "keep", data::source::SourceSpan::new(116, 133)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::String,
                    shape: data::type_::ValueShapeId(7),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::String(data::graph::StringLocalId(0))),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::String,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::String),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("native_loop", "keep", data::source::SourceSpan::new(116, 133)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::BitArray,
                    shape: data::type_::ValueShapeId(9),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
    ]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
