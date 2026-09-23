data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 4,
        program: data::ProgramTables {
            root: data::source::module_id(1),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("support"),
                    source_context: Some(data::source::SourceContext::from_static_block("support.gleam", r#"

@external(erlang, "ffi", "make_constant")
pub fn make_constant(value: a) -> fn() -> a
@external(erlang, "ffi", "make_adder")
pub fn make_adder(value: Int) -> fn(Int) -> Int
@external(erlang, "ffi", "wrap")
pub fn wrap(callback: fn(a) -> b) -> fn(a) -> b
"#)),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("library"),
                    source_context: Some(data::source::SourceContext::from_static_block("library.gleam", r#"

import support
fn apply(callback, value) { callback(value) }
pub fn make_native(offset: Int) -> fn(Int) -> Int { support.make_adder(offset) }
pub fn keep(adjust: fn(Int) -> Int) -> fn(Int) -> Int { adjust }
pub fn calculate(value: Int, adjust: fn(Int) -> Int) -> Int { adjust(value) }
pub fn function_list(items: List(fn(Int) -> Int)) -> List(fn(Int) -> Int) { items }
pub fn container(adjust: fn(Int) -> Int) -> #(fn(Int) -> Int, Result(fn(Int) -> Int, Nil)) { #(adjust, Ok(adjust)) }
pub fn maker() -> fn(Int) -> fn(Int) -> Int { fn(offset) { support.make_adder(offset) } }
pub fn result_function() -> fn(Int) -> Result(Int, Nil) { fn(value) { Ok(value) } }
pub fn picker() -> fn(List(Result(Int, Nil))) -> Int {
    fn(items) { case items { [Ok(value)] -> value _ -> 0 } }
}
pub fn run() {
    let add = support.make_adder(40)
    let constant = support.make_constant(2)
    apply(add, constant())
}
pub fn check() {
    let a = support.make_constant(True)
    let alias = a
    let b = support.make_constant(True)
    let list = support.make_constant([42])
    let is_answer = support.wrap(fn(value) { value == 42 })
    let increment = support.wrap(support.wrap(fn(value) { value + 1 }))
    let nested = support.wrap(fn(value) { support.make_constant(value) })
    a() && a == alias && a != b && list() == [42] && is_answer(increment(41)) && nested(42)() == 42
}
"#)),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("support/private"),
                    source_context: None,
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Function {
                id: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Custom(data::function::CustomFunctionFunctionId {
                    index: 0,
                    type_: data::type_::CustomFunctionType {
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                        },
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    },
                })),
                return_type: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                },
            }),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[]),
                    int_functions: data::Storage::Static(&[]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[]),
                    bit_array_functions: data::Storage::Static(&[]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledCustomFunctionBody {
                                _signature_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(1),
                                },
                                _body_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(1),
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
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(4),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                }),
                                            },
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                    ]),
                    external_functions: data::Storage::Static(&[]),
                    bool_functions: data::Storage::Static(&[]),
                    nil_functions: data::Storage::Static(&[]),
                    tuple_functions: data::Storage::Static(&[]),
                },
                list_returns: data::function::ListFunctionTables {
                    parameter_list_functions: data::Storage::Static(&[]),
                    int_list_functions: data::Storage::Static(&[]),
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
                    custom_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledCustomFunctionFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(5),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                    },
                                },
                                _type: data::type_::CustomFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                    },
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::CustomValueShape {
                                        type_id: data::type_::CustomTypeId(0),
                                        shape_id: data::type_::CustomValueShapeId(1),
                                    },
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
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::CustomFunction(data::graph::CustomFunctionLocal {
                                                        id: data::graph::CustomFunctionLocalId(0),
                                                        type_: data::type_::CustomFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                                            },
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(0),
                                                                shape_id: data::type_::CustomValueShapeId(1),
                                                            },
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(5),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                                    },
                                                    family: data::function::FunctionReturnFamily::Custom,
                                                    kind: data::graph::FunctionInstructionKind::Closure {
                                                        target: data::graph::FunctionTarget::Custom(data::function::CustomFunctionId {
                                                            index: 2,
                                                            return_shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(0),
                                                                shape_id: data::type_::CustomValueShapeId(1),
                                                            },
                                                        }),
                                                        captures: data::Storage::Static(&[]),
                                                    },
                                                }),
                                            },
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomFunctionLocal {
                                            id: data::graph::CustomFunctionLocalId(0),
                                            type_: data::type_::CustomFunctionType {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                                },
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueShapeId(0),
                                                ]),
                                                return_: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                    ]),
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
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..3,
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
                    3..4,
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
                        parameters: 0..0,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                    id: data::graph::CustomLocalId(0),
                                    shape: data::type_::CustomValueShape {
                                        type_id: data::type_::CustomTypeId(0),
                                        shape_id: data::type_::CustomValueShapeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..1,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::CustomFunction(data::graph::CustomFunctionLocal {
                                    id: data::graph::CustomFunctionLocalId(0),
                                    type_: data::type_::CustomFunctionType {
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(0),
                                        ]),
                                        return_: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    },
                                }),
                                shape: data::type_::ValueShapeId(3),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 1..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..2,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::Nil,
                            ]),
                        },
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Ok"),
                                native_tag: data::Text::Static("ok"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Int,
                                        shape: data::type_::ValueShapeId(0),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                                name: data::Text::Static("Error"),
                                native_tag: data::Text::Static("error"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Nil,
                                        shape: data::type_::ValueShapeId(1),
                                        refinement: data::type_::FieldRefinement::Argument(1),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                definitions: data::Storage::Static(&[]),
            },
            external_types: data::type_::ExternalTypeTable {
                types: data::Storage::Static(&[]),
            },
            value_shapes: data::type_::ValueShapeTable {
                shapes: data::Storage::Static(&[
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Nil,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                    },
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Nil,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                    }),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                    }),
                ]),
                custom_shapes: data::Storage::Static(&[
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                ]),
            },
        },
        entries: data::program::LibraryFunctionEntries {
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
            functions: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::program::LibraryCallableEntry {
                        function: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Custom(data::function::CustomFunctionFunctionId {
                            index: 0,
                            type_: data::type_::CustomFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                        })),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                        },
                    },
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                            },
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
                },
            ]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("result_function"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::Nil,
                            ]),
                        })),
                    })),
                },
                slot: 0,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Custom,
                index: 0,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "constant", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static(""),
                    module: data::Text::Static("gleam"),
                    name: data::Text::Static("Result"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Nil,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                            data::type_::TypeMetadata::Nil,
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(2),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                            id: data::graph::CustomLocalId(0),
                            shape: data::type_::CustomValueShape {
                                type_id: data::type_::CustomTypeId(0),
                                shape_id: data::type_::CustomValueShapeId(0),
                            },
                        }),
                        shape: data::type_::ValueShapeId(2),
                    },
                ]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::Nil,
                            ]),
                        }), data::type_::CustomTypeId(0)),
                    ]),
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
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                callable: true,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Custom,
                index: 1,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "wrap", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static(""),
                    module: data::Text::Static("gleam"),
                    name: data::Text::Static("Result"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Nil,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                            data::type_::TypeMetadata::Nil,
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(2),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::CustomFunction(data::graph::CustomFunctionLocal {
                            id: data::graph::CustomFunctionLocalId(0),
                            type_: data::type_::CustomFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                        }),
                        shape: data::type_::ValueShapeId(3),
                    },
                ]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::Nil,
                            ]),
                        }), data::type_::CustomTypeId(0)),
                    ]),
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
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                callable: true,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(1),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
    ]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[
        data::program::LibraryNativeConstruction {
            declaration: data::host::CallableRegistration {
                package: data::Text::Static("support"),
                module: data::Text::Static("support/private"),
                name: data::Text::Static("constant"),
                arguments: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            parameter_count: 2,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Ok"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Error"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(1),
                                        },
                                    ]),
                                },
                            ]),
                            shared: false,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                            data::host::RegistrationType::Nil,
                        ]),
                    },
                ]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        shared: false,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Int,
                        data::host::RegistrationType::Nil,
                    ]),
                },
                returns_value: true,
            },
            construction: data::host::HostCallableConstruction {
                target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Custom(data::function::CustomFunctionId {
                    index: 0,
                    return_shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(0),
                    },
                })),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                },
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                            id: data::graph::CustomLocalId(0),
                            shape: data::type_::CustomValueShape {
                                type_id: data::type_::CustomTypeId(0),
                                shape_id: data::type_::CustomValueShapeId(0),
                            },
                        }),
                        shape: data::type_::ValueShapeId(2),
                    },
                ]),
            },
            invocation: data::program::LibraryCallable {
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                },
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
            captures: data::program::LibraryInputConstructions {
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
        },
        data::program::LibraryNativeConstruction {
            declaration: data::host::CallableRegistration {
                package: data::Text::Static("support"),
                module: data::Text::Static("support/private"),
                name: data::Text::Static("constant"),
                arguments: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            parameter_count: 2,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Ok"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Error"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(1),
                                        },
                                    ]),
                                },
                            ]),
                            shared: false,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                            data::host::RegistrationType::Nil,
                        ]),
                    },
                ]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        shared: false,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Int,
                        data::host::RegistrationType::Nil,
                    ]),
                },
                returns_value: true,
            },
            construction: data::host::HostCallableConstruction {
                target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Custom(data::function::CustomFunctionId {
                    index: 0,
                    return_shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(0),
                    },
                })),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                },
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                            id: data::graph::CustomLocalId(0),
                            shape: data::type_::CustomValueShape {
                                type_id: data::type_::CustomTypeId(0),
                                shape_id: data::type_::CustomValueShapeId(0),
                            },
                        }),
                        shape: data::type_::ValueShapeId(2),
                    },
                ]),
            },
            invocation: data::program::LibraryCallable {
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                },
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
            captures: data::program::LibraryInputConstructions {
                variants: data::Storage::Static(&[
                    [
                        data::type_::CustomConstructorId {
                            type_id: data::type_::CustomTypeId(0),
                            index: 0,
                        },
                        data::type_::CustomConstructorId {
                            type_id: data::type_::CustomTypeId(0),
                            index: 1,
                        },
                    ],
                ]),
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
        },
        data::program::LibraryNativeConstruction {
            declaration: data::host::CallableRegistration {
                package: data::Text::Static("support"),
                module: data::Text::Static("support/private"),
                name: data::Text::Static("wrap"),
                arguments: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Custom {
                            schema: data::host::CustomSchema {
                                package: data::Text::Static(""),
                                module: data::Text::Static("gleam"),
                                name: data::Text::Static("Result"),
                                parameter_count: 2,
                                constructors: data::Storage::Static(&[
                                    data::host::ConstructorSchema {
                                        name: data::Text::Static("Ok"),
                                        fields: data::Storage::Static(&[
                                            data::host::FieldSchema {
                                                label: None,
                                                type_: data::host::SchemaType::Parameter(0),
                                            },
                                        ]),
                                    },
                                    data::host::ConstructorSchema {
                                        name: data::Text::Static("Error"),
                                        fields: data::Storage::Static(&[
                                            data::host::FieldSchema {
                                                label: None,
                                                type_: data::host::SchemaType::Parameter(1),
                                            },
                                        ]),
                                    },
                                ]),
                                shared: false,
                            },
                            arguments: data::Storage::Static(&[
                                data::host::RegistrationType::Int,
                                data::host::RegistrationType::Nil,
                            ]),
                        }),
                    },
                ]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        shared: false,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Int,
                        data::host::RegistrationType::Nil,
                    ]),
                },
                returns_value: true,
            },
            construction: data::host::HostCallableConstruction {
                target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Custom(data::function::CustomFunctionId {
                    index: 1,
                    return_shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(0),
                    },
                })),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                },
                parameters: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                        shape: data::type_::ValueShapeId(0),
                    },
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::CustomFunction(data::graph::CustomFunctionLocal {
                            id: data::graph::CustomFunctionLocalId(0),
                            type_: data::type_::CustomFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                        }),
                        shape: data::type_::ValueShapeId(3),
                    },
                ]),
            },
            invocation: data::program::LibraryCallable {
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
                },
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
            captures: data::program::LibraryInputConstructions {
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
        },
    ]),
}
