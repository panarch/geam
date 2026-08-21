use super::{call_args, custom, function, list, lower_function_call, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{DraftCursor, DraftFlow, DraftGraph, DraftValueRef};
use crate::plan::execution::lowering::specialization::{
    Representability, StoredValueShape, UninhabitedCustomValueShape, UninhabitedTupleValueShape,
    UninhabitedValueShape, ValueInhabitation,
};
use crate::plan::module;

type Lowered = Representability<DraftFlow<DraftValueRef>>;

pub(in crate::plan::execution::lowering) fn generic_expr(
    expression: &module::GenericExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered {
    let shape = context.concrete_parameter(expression.parameter());
    match context
        .representations
        .representation(&shape)
        .into_representability()
    {
        Representability::Uninhabited => {
            never_expr(expression, cursor, graph, context).map(|()| DraftFlow::Diverged)
        }
        Representability::Inhabited(shape) => {
            stored_expr(expression, &shape, cursor, graph, context)
        }
    }
}

pub(super) fn stored_expr(
    expression: &module::GenericExpr,
    shape: &StoredValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered {
    use module::GenericExprKind as E;

    match expression.kind() {
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().get(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Generic,
                local.id().0,
            ));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                cursor,
                value: args,
            } => direct_call(shape, function, args, site, cursor, graph, context),
        }),
        E::FunctionCall {
            function: value,
            args,
            site,
        } => {
            let function_shape = context.concrete_function_shape(&value.shape());
            lower_function_call(
                args,
                cursor,
                graph,
                context,
                |cursor, graph, context| {
                    function::generic_executable_function_expr(
                        value,
                        &function_shape,
                        shape,
                        cursor,
                        graph,
                        context,
                    )
                },
                |cursor, graph, context| {
                    function::evaluated_generic_function_expr(value, cursor, graph, context)
                },
                |cursor, function, args, graph, context| {
                    function_call(shape, function, args, site, cursor, graph, context)
                },
            )
        }
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                cursor,
                value: source,
            } => {
                let (cursor, value) = tuple_index(shape, source, *index, cursor, graph, context);
                DraftFlow::value(cursor, value)
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    cursor,
                    value: source,
                } => {
                    let (cursor, value) =
                        custom_field(shape, source, access.index(), cursor, graph, context);
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::generic_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                cursor,
                value: source,
            } => list_index(shape, source, *index, cursor, graph, context),
        }),
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, shape.clone()),
            |cursor, graph, context| stored_expr(true_, shape, cursor, graph, context),
            |cursor, graph, context| stored_expr(false_, shape, cursor, graph, context),
            Clone::clone,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, shape.clone()),
            |branch, cursor, graph, context| stored_expr(branch, shape, cursor, graph, context),
            Clone::clone,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, shape.clone()),
            |branch, cursor, graph, context| stored_expr(branch, shape, cursor, graph, context),
            Clone::clone,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, shape.clone()),
            |branch, cursor, graph, context| stored_expr(branch, shape, cursor, graph, context),
            Clone::clone,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    stored_expr(return_, shape, cursor, graph, context)
                }
            }),
    }
}

fn direct_call(
    shape: &StoredValueShape,
    function: &module::FunctionInstantiation,
    args: Vec<DraftValueRef>,
    site: &crate::plan::HostCallSite,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered {
    match shape {
        StoredValueShape::Int => context.int_function_id(function).map(|function| {
            let value = graph.int_instruction(
                &mut cursor,
                super::super::instruction::DraftIntInstruction::Call {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }),
        StoredValueShape::Float => context.float_function_id(function).map(|function| {
            let value = graph.float_instruction(
                &mut cursor,
                super::super::instruction::DraftFloatInstruction::Call {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }),
        StoredValueShape::String => context.string_function_id(function).map(|function| {
            let value = graph.string_instruction(
                &mut cursor,
                super::super::instruction::DraftStringInstruction::Call {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }),
        StoredValueShape::BitArray => context.bit_array_function_id(function).map(|function| {
            let value = graph.bit_array_instruction(
                &mut cursor,
                super::super::instruction::DraftBitArrayInstruction::Call {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }),
        StoredValueShape::UtfCodepoint => {
            context.utf_codepoint_function_id(function).map(|function| {
                let value = graph.utf_codepoint_instruction(
                    &mut cursor,
                    super::super::instruction::DraftUtfCodepointInstruction::Call {
                        function,
                        args,
                        site: site.clone(),
                    },
                );
                DraftFlow::value(cursor, value.erase())
            })
        }
        StoredValueShape::Custom(shape) => {
            context.custom_function_id(function, shape).map(|function| {
                let value = graph.custom_instruction(
                    &mut cursor,
                    shape.clone(),
                    super::super::instruction::DraftCustomInstruction::Call {
                        function,
                        args,
                        site: site.clone(),
                    },
                );
                DraftFlow::value(cursor, value.erase())
            })
        }
        StoredValueShape::External(shape) => {
            context
                .external_function_id(function, shape)
                .map(|function| {
                    let value = graph.external_instruction(
                        &mut cursor,
                        shape.clone(),
                        super::super::instruction::DraftExternalInstruction::Call {
                            function,
                            args,
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, value.erase())
                })
        }
        StoredValueShape::Bool => context.bool_function_id(function).map(|function| {
            let value = graph.bool_instruction(
                &mut cursor,
                super::super::instruction::DraftBoolInstruction::Call {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }),
        StoredValueShape::Nil => context.nil_function_id(function).map(|function| {
            let value = graph.nil_instruction(
                &mut cursor,
                super::super::instruction::DraftNilInstruction::Call {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }),
        StoredValueShape::Tuple(elements) => context.tuple_function_id(function).map(|function| {
            let value = graph.tuple_instruction(
                &mut cursor,
                elements.clone(),
                super::super::instruction::DraftTupleInstruction::Call {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }),
        StoredValueShape::List(item) => {
            list::generic_direct_call(item, function, args, site, cursor, graph, context)
                .map(|flow| flow.map(|value| value.erase()))
        }
        StoredValueShape::Function(shape) => {
            context
                .function_function_id(function, shape)
                .map(|function| {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.as_ref().clone(),
                        super::super::instruction::DraftFunctionInstruction::Call {
                            function,
                            args,
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, value.erase())
                })
        }
    }
}

fn function_call(
    shape: &StoredValueShape,
    function: super::super::DraftFunction,
    args: Vec<DraftValueRef>,
    site: &crate::plan::HostCallSite,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> DraftFlow<DraftValueRef> {
    match shape {
        StoredValueShape::Int => {
            let value = graph.int_instruction(
                &mut cursor,
                super::super::instruction::DraftIntInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::Float => {
            let value = graph.float_instruction(
                &mut cursor,
                super::super::instruction::DraftFloatInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::String => {
            let value = graph.string_instruction(
                &mut cursor,
                super::super::instruction::DraftStringInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::BitArray => {
            let value = graph.bit_array_instruction(
                &mut cursor,
                super::super::instruction::DraftBitArrayInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::UtfCodepoint => {
            let value = graph.utf_codepoint_instruction(
                &mut cursor,
                super::super::instruction::DraftUtfCodepointInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::Custom(shape) => {
            let value = graph.custom_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftCustomInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::External(shape) => {
            let value = graph.external_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftExternalInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::Bool => {
            let value = graph.bool_instruction(
                &mut cursor,
                super::super::instruction::DraftBoolInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::Nil => {
            let value = graph.nil_instruction(
                &mut cursor,
                super::super::instruction::DraftNilInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::Tuple(elements) => {
            let value = graph.tuple_instruction(
                &mut cursor,
                elements.clone(),
                super::super::instruction::DraftTupleInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::List(item) => {
            let value = list::generic_function_call(
                item,
                function,
                args,
                site,
                &mut cursor,
                graph,
                context,
            );
            DraftFlow::value(cursor, value.erase())
        }
        StoredValueShape::Function(shape) => {
            let value = graph.function_instruction(
                &mut cursor,
                shape.as_ref().clone(),
                super::super::instruction::DraftFunctionInstruction::FunctionCall {
                    function,
                    args,
                    site: site.clone(),
                },
            );
            DraftFlow::value(cursor, value.erase())
        }
    }
}

pub(in crate::plan::execution::lowering) fn tuple_index(
    shape: &StoredValueShape,
    source: super::super::DraftTuple,
    index: usize,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> (DraftCursor, DraftValueRef) {
    let value = match shape {
        StoredValueShape::Int => graph
            .int_instruction(
                &mut cursor,
                super::super::instruction::DraftIntInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Float => graph
            .float_instruction(
                &mut cursor,
                super::super::instruction::DraftFloatInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::String => graph
            .string_instruction(
                &mut cursor,
                super::super::instruction::DraftStringInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::BitArray => graph
            .bit_array_instruction(
                &mut cursor,
                super::super::instruction::DraftBitArrayInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::UtfCodepoint => graph
            .utf_codepoint_instruction(
                &mut cursor,
                super::super::instruction::DraftUtfCodepointInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Custom(shape) => graph
            .custom_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftCustomInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::External(shape) => graph
            .external_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftExternalInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Bool => graph
            .bool_instruction(
                &mut cursor,
                super::super::instruction::DraftBoolInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Nil => graph
            .nil_instruction(
                &mut cursor,
                super::super::instruction::DraftNilInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Tuple(elements) => graph
            .tuple_instruction(
                &mut cursor,
                elements.clone(),
                super::super::instruction::DraftTupleInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::List(item) => {
            list::generic_tuple_index(item, source, index, &mut cursor, graph, context).erase()
        }
        StoredValueShape::Function(shape) => graph
            .function_instruction(
                &mut cursor,
                shape.as_ref().clone(),
                super::super::instruction::DraftFunctionInstruction::TupleIndex {
                    tuple: source,
                    index,
                },
            )
            .erase(),
    };
    (cursor, value)
}

pub(in crate::plan::execution::lowering) fn custom_field(
    shape: &StoredValueShape,
    source: super::super::DraftCustom,
    index: usize,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> (DraftCursor, DraftValueRef) {
    let value = match shape {
        StoredValueShape::Int => graph
            .int_instruction(
                &mut cursor,
                super::super::instruction::DraftIntInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::Float => graph
            .float_instruction(
                &mut cursor,
                super::super::instruction::DraftFloatInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::String => graph
            .string_instruction(
                &mut cursor,
                super::super::instruction::DraftStringInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::BitArray => graph
            .bit_array_instruction(
                &mut cursor,
                super::super::instruction::DraftBitArrayInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::UtfCodepoint => graph
            .utf_codepoint_instruction(
                &mut cursor,
                super::super::instruction::DraftUtfCodepointInstruction::CustomField {
                    source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Custom(shape) => graph
            .custom_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftCustomInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::External(shape) => graph
            .external_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftExternalInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::Bool => graph
            .bool_instruction(
                &mut cursor,
                super::super::instruction::DraftBoolInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::Nil => graph
            .nil_instruction(
                &mut cursor,
                super::super::instruction::DraftNilInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::Tuple(elements) => graph
            .tuple_instruction(
                &mut cursor,
                elements.clone(),
                super::super::instruction::DraftTupleInstruction::CustomField { source, index },
            )
            .erase(),
        StoredValueShape::List(item) => {
            list::generic_custom_field(item, source, index, &mut cursor, graph, context).erase()
        }
        StoredValueShape::Function(shape) => graph
            .function_instruction(
                &mut cursor,
                shape.as_ref().clone(),
                super::super::instruction::DraftFunctionInstruction::CustomField { source, index },
            )
            .erase(),
    };
    (cursor, value)
}

fn list_index(
    shape: &StoredValueShape,
    source: super::super::DraftList,
    index: usize,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> DraftFlow<DraftValueRef> {
    let value = match shape {
        StoredValueShape::Int => graph
            .int_instruction(
                &mut cursor,
                super::super::instruction::DraftIntInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Float => graph
            .float_instruction(
                &mut cursor,
                super::super::instruction::DraftFloatInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::String => graph
            .string_instruction(
                &mut cursor,
                super::super::instruction::DraftStringInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::BitArray => graph
            .bit_array_instruction(
                &mut cursor,
                super::super::instruction::DraftBitArrayInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::UtfCodepoint => graph
            .utf_codepoint_instruction(
                &mut cursor,
                super::super::instruction::DraftUtfCodepointInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Custom(shape) => graph
            .custom_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftCustomInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::External(shape) => graph
            .external_instruction(
                &mut cursor,
                shape.clone(),
                super::super::instruction::DraftExternalInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Bool => graph
            .bool_instruction(
                &mut cursor,
                super::super::instruction::DraftBoolInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Nil => graph
            .nil_instruction(
                &mut cursor,
                super::super::instruction::DraftNilInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::Tuple(elements) => graph
            .tuple_instruction(
                &mut cursor,
                elements.clone(),
                super::super::instruction::DraftTupleInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
        StoredValueShape::List(item) => {
            list::generic_list_index(item, source, index, &mut cursor, graph, context).erase()
        }
        StoredValueShape::Function(shape) => graph
            .function_instruction(
                &mut cursor,
                shape.as_ref().clone(),
                super::super::instruction::DraftFunctionInstruction::ListIndex {
                    list: source,
                    index,
                },
            )
            .erase(),
    };
    DraftFlow::value(cursor, value)
}

pub(in crate::plan::execution::lowering) fn never_expr(
    expression: &module::GenericExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    use module::GenericExprKind as E;

    match expression.kind() {
        E::LocalGet { .. } | E::ListIndex { .. } => Representability::Uninhabited,
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(()),
            DraftFlow::Value {
                cursor,
                value: args,
            } => context.never_function_id(function).map(|function| {
                graph.finish_never_call(cursor, function, args, site.clone());
            }),
        }),
        E::FunctionCall {
            function: value,
            args,
            site,
        } => {
            let shape = context.concrete_function_shape(&value.shape());
            function::generic_never_function_expr(value, &shape, cursor, graph, context).and_then(
                |flow| match flow {
                    DraftFlow::Diverged => Representability::Inhabited(()),
                    DraftFlow::Value {
                        cursor,
                        value: function,
                    } => call_args(args, cursor, graph, context).map(|flow| match flow {
                        DraftFlow::Diverged => (),
                        DraftFlow::Value {
                            cursor,
                            value: args,
                        } => {
                            graph.finish_never_function_call(
                                cursor,
                                function.value().clone(),
                                args,
                                site.clone(),
                            );
                        }
                    }),
                },
            )
        }
        E::TupleIndex { tuple: source, .. } => tuple::tuple_expr(source, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { .. } => Representability::Uninhabited,
            }),
        E::CustomField(access) => custom::custom_expr(access.source(), cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { .. } => Representability::Uninhabited,
            }),
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| ()),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => never_bool_case(subject, true_, false_, cursor, graph, context),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => never_int_case(subject, clauses, fallback, cursor, graph, context),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => never_string_case(subject, clauses, fallback, cursor, graph, context),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => never_float_case(subject, clauses, fallback, cursor, graph, context),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { cursor, value: () } => {
                    never_expr(return_, cursor, graph, context)
                }
            }),
    }
}

fn never_bool_case(
    subject: &module::BoolExpr,
    true_: &module::GenericExpr,
    false_: &module::GenericExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    super::bool::bool_paths(subject, cursor, graph, context).and_then(|paths| match paths {
        super::bool::BoolPaths::Diverged => Representability::Inhabited(()),
        super::bool::BoolPaths::True(cursor) => never_expr(true_, cursor, graph, context),
        super::bool::BoolPaths::False(cursor) => never_expr(false_, cursor, graph, context),
        super::bool::BoolPaths::Both {
            true_: true_cursor,
            false_: false_cursor,
        } => never_expr(true_, true_cursor, graph, context).and_then(|true_| {
            never_expr(false_, false_cursor, graph, context).map(|false_| {
                let ((), ()) = (true_, false_);
            })
        }),
    })
}

fn never_int_case(
    subject: &module::IntExpr,
    clauses: &[(num_bigint::BigInt, module::GenericExpr)],
    fallback: &module::GenericExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    super::int_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(()),
        DraftFlow::Value {
            cursor,
            value: subject,
        } => {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            graph.finish_int_switch(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match never_expr(branch, cursor, graph, context) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            never_expr(fallback, fallback_cursor, graph, context)
        }
    })
}

fn never_string_case(
    subject: &module::StringExpr,
    clauses: &[(ecow::EcoString, module::GenericExpr)],
    fallback: &module::GenericExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    super::string_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(()),
        DraftFlow::Value {
            cursor,
            value: subject,
        } => {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            graph.finish_string_switch(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match never_expr(branch, cursor, graph, context) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            never_expr(fallback, fallback_cursor, graph, context)
        }
    })
}

fn never_float_case(
    subject: &module::FloatExpr,
    clauses: &[(f64, module::GenericExpr)],
    fallback: &module::GenericExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    super::float_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(()),
        DraftFlow::Value {
            cursor,
            value: subject,
        } => {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            graph.finish_float_switch(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (*pattern, branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match never_expr(branch, cursor, graph, context) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            never_expr(fallback, fallback_cursor, graph, context)
        }
    })
}

pub(in crate::plan::execution::lowering) fn tuple_never_expr(
    expression: &module::TupleExpr,
    proof: &UninhabitedTupleValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    use module::TupleExprKind as E;

    match expression.kind() {
        E::Value(values) => diverging_values(values, proof.diverging(), cursor, graph, context),
        E::Constant(_) | E::LocalGet { .. } | E::ListIndex { .. } => Representability::Uninhabited,
        E::Call {
            function,
            args,
            site,
        } => never_direct_call(function, args, site, cursor, graph, context),
        E::FunctionCall {
            function: value,
            args,
            site,
        } => function::tuple_never_function_expr(value, cursor, graph, context).and_then(|flow| {
            match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value {
                    cursor,
                    value: function,
                } => never_function_call(
                    function.value().clone(),
                    args,
                    site,
                    cursor,
                    graph,
                    context,
                ),
            }
        }),
        E::TupleIndex { tuple: source, .. } => tuple::tuple_expr(source, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { .. } => Representability::Uninhabited,
            }),
        E::CustomField(access) => custom::custom_expr(access.source(), cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { .. } => Representability::Uninhabited,
            }),
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| ()),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => uninhabited_bool_case(
            subject,
            true_,
            false_,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                tuple_never_expr(branch, proof, cursor, graph, context)
            },
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => uninhabited_int_case(
            subject,
            clauses,
            fallback,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                tuple_never_expr(branch, proof, cursor, graph, context)
            },
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => uninhabited_string_case(
            subject,
            clauses,
            fallback,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                tuple_never_expr(branch, proof, cursor, graph, context)
            },
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => uninhabited_float_case(
            subject,
            clauses,
            fallback,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                tuple_never_expr(branch, proof, cursor, graph, context)
            },
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { cursor, value: () } => {
                    tuple_never_expr(return_, proof, cursor, graph, context)
                }
            }),
    }
}

pub(in crate::plan::execution::lowering) fn custom_never_expr(
    expression: &module::CustomExpr,
    proof: &UninhabitedCustomValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    custom_never_expr_kind(expression.kind(), proof, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn custom_never_expr_kind(
    kind: &module::CustomExprKind,
    proof: &UninhabitedCustomValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    use module::CustomExprKind as E;

    match kind {
        E::Constructor(construction) => diverging_values(
            construction.fields(),
            proof.diverging_field(construction.constructor().index()),
            cursor,
            graph,
            context,
        ),
        E::Constant(_) | E::LocalGet { .. } | E::ListIndex { .. } => Representability::Uninhabited,
        E::Call {
            function,
            args,
            site,
        } => never_direct_call(function, args, site, cursor, graph, context),
        E::FunctionCall(call) => {
            function::custom_never_function_expr(call.function(), cursor, graph, context).and_then(
                |flow| match flow {
                    DraftFlow::Diverged => Representability::Inhabited(()),
                    DraftFlow::Value {
                        cursor,
                        value: function,
                    } => never_function_call(
                        function.value().clone(),
                        call.arguments(),
                        call.site(),
                        cursor,
                        graph,
                        context,
                    ),
                },
            )
        }
        E::TupleIndex { tuple: source, .. } => tuple::tuple_expr(source, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { .. } => Representability::Uninhabited,
            }),
        E::CustomField(access) => custom::custom_expr(access.source(), cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { .. } => Representability::Uninhabited,
            }),
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| ()),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => uninhabited_bool_case(
            subject,
            true_,
            false_,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                custom_never_expr_kind(branch, proof, cursor, graph, context)
            },
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => uninhabited_int_case(
            subject,
            clauses,
            fallback,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                custom_never_expr_kind(branch, proof, cursor, graph, context)
            },
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => uninhabited_string_case(
            subject,
            clauses,
            fallback,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                custom_never_expr_kind(branch, proof, cursor, graph, context)
            },
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => uninhabited_float_case(
            subject,
            clauses,
            fallback,
            cursor,
            graph,
            context,
            |branch, cursor, graph, context| {
                custom_never_expr_kind(branch, proof, cursor, graph, context)
            },
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(()),
                DraftFlow::Value { cursor, value: () } => {
                    custom_never_expr_kind(return_, proof, cursor, graph, context)
                }
            }),
    }
}

fn never_direct_call(
    function: &module::FunctionInstantiation,
    args: &[module::CallArg],
    site: &crate::plan::HostCallSite,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    call_args(args, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(()),
        DraftFlow::Value {
            cursor,
            value: args,
        } => context.never_function_id(function).map(|function| {
            graph.finish_never_call(cursor, function, args, site.clone());
        }),
    })
}

fn never_function_call(
    function: super::super::DraftFunction,
    args: &[module::CallArg],
    site: &crate::plan::HostCallSite,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    call_args(args, cursor, graph, context).map(|flow| match flow {
        DraftFlow::Diverged => (),
        DraftFlow::Value {
            cursor,
            value: args,
        } => {
            graph.finish_never_function_call(cursor, function, args, site.clone());
        }
    })
}

fn diverging_values(
    values: &[module::Expr],
    diverging: usize,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    for (index, value) in values.iter().enumerate() {
        if index == diverging {
            let shape = context.concrete_value_shape(value.value_shape());
            return match context.representations.inhabitation(&shape) {
                ValueInhabitation::Inhabited(_) => Representability::Uninhabited,
                ValueInhabitation::Uninhabited(proof) => {
                    uninhabited_expr(value, &proof, cursor, graph, context)
                }
            };
        }
        match super::expr(value, cursor, graph, context) {
            Representability::Uninhabited => return Representability::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => {
                return Representability::Inhabited(());
            }
            Representability::Inhabited(DraftFlow::Value { cursor: next, .. }) => cursor = next,
        }
    }
    Representability::Uninhabited
}

fn uninhabited_expr(
    expression: &module::Expr,
    proof: &UninhabitedValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    match (expression.kind(), proof) {
        (module::ExprKind::Generic(expression), _) => {
            never_expr(expression, cursor, graph, context)
        }
        (module::ExprKind::Tuple(expression), UninhabitedValueShape::Tuple(proof)) => {
            tuple_never_expr(expression, proof, cursor, graph, context)
        }
        (module::ExprKind::Custom(expression), UninhabitedValueShape::Custom(proof)) => {
            custom_never_expr(expression, proof, cursor, graph, context)
        }
        _ => Representability::Uninhabited,
    }
}

fn uninhabited_bool_case<Branch>(
    subject: &module::BoolExpr,
    true_branch: &Branch,
    false_branch: &Branch,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    lower: impl Copy
    + Fn(
        &Branch,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<()>,
) -> Representability<()>
where
{
    super::bool::bool_paths(subject, cursor, graph, context).and_then(|paths| match paths {
        super::bool::BoolPaths::Diverged => Representability::Inhabited(()),
        super::bool::BoolPaths::True(cursor) => lower(true_branch, cursor, graph, context),
        super::bool::BoolPaths::False(cursor) => lower(false_branch, cursor, graph, context),
        super::bool::BoolPaths::Both { true_, false_ } => lower(true_branch, true_, graph, context)
            .and_then(|()| lower(false_branch, false_, graph, context)),
    })
}

fn uninhabited_int_case<Branch>(
    subject: &module::IntExpr,
    clauses: &[(num_bigint::BigInt, Branch)],
    fallback: &Branch,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    lower: impl Copy
    + Fn(
        &Branch,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<()>,
) -> Representability<()>
where
{
    super::int_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(()),
        DraftFlow::Value {
            cursor,
            value: subject,
        } => {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            graph.finish_int_switch(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower(branch, cursor, graph, context) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            lower(fallback, fallback_cursor, graph, context)
        }
    })
}

fn uninhabited_string_case<Branch>(
    subject: &module::StringExpr,
    clauses: &[(ecow::EcoString, Branch)],
    fallback: &Branch,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    lower: impl Copy
    + Fn(
        &Branch,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<()>,
) -> Representability<()>
where
{
    super::string_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(()),
        DraftFlow::Value {
            cursor,
            value: subject,
        } => {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            graph.finish_string_switch(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower(branch, cursor, graph, context) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            lower(fallback, fallback_cursor, graph, context)
        }
    })
}

fn uninhabited_float_case<Branch>(
    subject: &module::FloatExpr,
    clauses: &[(f64, Branch)],
    fallback: &Branch,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    lower: impl Copy
    + Fn(
        &Branch,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<()>,
) -> Representability<()>
where
{
    super::float_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(()),
        DraftFlow::Value {
            cursor,
            value: subject,
        } => {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            graph.finish_float_switch(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (*pattern, branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower(branch, cursor, graph, context) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            lower(fallback, fallback_cursor, graph, context)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{diverging_values, never_expr, stored_expr};
    use crate::plan::execution::lowering::graph::draft::DraftGraphBuilder;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{
        Representability, StoredValueShape, UninhabitedValueShape,
    };
    use crate::plan::{self, TypeParameterId, ValueShape, ValueType};

    #[derive(Debug, PartialEq, Eq)]
    enum FlowOutcome {
        Uninhabited,
        Diverged,
        Value,
    }

    fn flow_outcome<T>(flow: Representability<DraftFlow<T>>) -> FlowOutcome {
        match flow {
            Representability::Uninhabited => FlowOutcome::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => FlowOutcome::Diverged,
            Representability::Inhabited(DraftFlow::Value { .. }) => FlowOutcome::Value,
        }
    }

    fn source_stop() -> plan::PanicExpr {
        plan::PanicExpr::panic_at(None, plan::PanicSite::unknown())
    }

    #[test]
    fn generic_family_specialization_executes_every_stored_shape() {
        let source = include_str!(
            "../../../../../../../tests/fixtures/execution/functions/generic_family_specialization.gleam"
        );
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("generic family fixture should compile");
        let module = crate::plan_module(typed).expect("generic family fixture should plan");
        let execution = crate::ExecutionPlan::from_module_plan(module);

        assert_eq!(
            crate::run_main(&execution, &mut Vec::new()),
            Ok(crate::Value::Int(0.into())),
        );
    }

    #[test]
    fn recursive_never_value_handoffs_preserve_every_reachable_owner_path() {
        let source = include_str!(
            "../../../../../../../tests/fixtures/execution/functions/generic_recursive_never_value_handoffs.gleam"
        );
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("recursive never value fixture should compile");
        let module = crate::plan_module(typed).expect("recursive never value fixture should plan");
        let execution = crate::ExecutionPlan::from_module_plan(module);

        assert_eq!(
            crate::run_main(&execution, &mut Vec::new()),
            Ok(crate::Value::Tuple(vec![crate::Value::Bool(true); 28])),
        );
    }

    fn boxed_definition(name: plan::CustomTypeName) -> plan::CustomTypeDefinition {
        plan::CustomTypeDefinition::new(
            name,
            plan::CustomTypePublicity::Private,
            false,
            vec![plan::CustomTypeParameterId(0)],
            vec![plan::CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![plan::CustomFieldDefinition::new(
                    None,
                    plan::CustomTypeTemplate::Parameter(plan::CustomTypeParameterId(0)),
                )],
            )],
        )
    }

    #[test]
    fn uninhabited_generic_owners_preserve_only_reachable_source_stops() {
        let parameter = TypeParameterId(0);
        let custom_name = plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let custom_definition = plan::CustomTypeDefinition::new(
            custom_name.clone(),
            plan::CustomTypePublicity::Private,
            false,
            vec![plan::CustomTypeParameterId(0)],
            vec![plan::CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![plan::CustomFieldDefinition::new(
                    None,
                    plan::CustomTypeTemplate::Parameter(plan::CustomTypeParameterId(0)),
                )],
            )],
        );
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            custom_definition,
        ]);
        let local = plan::GenericExpr::local_get(
            plan::GenericLocal::new(plan::GenericLocalId(0), parameter),
            "value".into(),
        );
        let empty_list = plan::ListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
            .expect("an empty generic list should preserve its item parameter")
            .into_generic()
            .expect("a parameter item type should create a generic list expression");
        let list_index = plan::GenericExpr::list_index(empty_list, 0);
        let panic = || plan::PanicExpr::panic_at(None, plan::PanicSite::unknown());
        let diverging_argument = plan::CallArg::new(plan::Expr::generic(plan::GenericExpr::panic(
            parameter,
            panic(),
        )));
        let diverging_call = plan::GenericExpr::call(
            parameter,
            plan::monomorphic_function_instantiation(
                1,
                plan::FunctionShape::new(
                    vec![plan::ValueShape::Parameter(parameter)],
                    plan::ValueShape::Parameter(parameter),
                ),
            ),
            vec![diverging_argument],
        );
        let erased_argument = plan::CallArg::new(plan::Expr::generic(local.clone()));
        let erased_direct_call = plan::GenericExpr::call(
            parameter,
            plan::monomorphic_function_instantiation(
                1,
                plan::FunctionShape::new(
                    vec![plan::ValueShape::Parameter(parameter)],
                    plan::ValueShape::Parameter(parameter),
                ),
            ),
            vec![erased_argument.clone()],
        );
        let erased_function_call = plan::GenericExpr::function_call(
            plan::GenericFunctionExpr::panic(
                panic(),
                plan::GenericFunctionType::new(
                    vec![plan::ValueShape::Parameter(parameter)],
                    parameter,
                ),
            ),
            vec![erased_argument],
        );
        let tuple_projection = plan::GenericExpr::tuple_index(
            parameter,
            plan::TupleExpr::local_get(
                plan::TupleLocalId(0),
                "tuple".into(),
                vec![ValueType::Parameter(parameter)],
            ),
            0,
        );
        let custom_shape = plan::CustomValueShape::new(
            custom_name.clone(),
            vec![plan::ValueShape::Parameter(parameter)],
            plan::CustomConstructorRefinement::Exact(0),
        );
        let custom_source = plan::CustomExpr::local_get(
            plan::CustomLocal::from_shape(plan::CustomLocalId(0), custom_shape),
            "boxed".into(),
        );
        let custom_projection = plan::GenericExpr::custom_field(
            parameter,
            plan::CustomFieldAccess::new(custom_source, 0, None),
        );
        let diverging_block = plan::GenericExpr::block(
            vec![plan::Step::evaluate(plan::Expr::generic(
                plan::GenericExpr::panic(parameter, panic()),
            ))],
            local.clone(),
        );

        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        assert_eq!(
            never_expr(&local, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&list_index, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&tuple_projection, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&custom_projection, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&diverging_call, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&erased_direct_call, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&erased_function_call, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&diverging_block, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );
    }

    #[test]
    fn compound_uninhabited_values_lower_only_their_reachable_prefix() {
        let parameter = TypeParameterId(0);
        let custom_name = plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let custom_definition = plan::CustomTypeDefinition::new(
            custom_name.clone(),
            plan::CustomTypePublicity::Private,
            false,
            vec![plan::CustomTypeParameterId(0)],
            vec![plan::CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![plan::CustomFieldDefinition::new(
                    None,
                    plan::CustomTypeTemplate::Parameter(plan::CustomTypeParameterId(0)),
                )],
            )],
        );
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            custom_definition,
        ]);
        let panic = || plan::PanicExpr::panic_at(None, plan::PanicSite::unknown());
        let diverging_step = plan::Step::evaluate(plan::Expr::generic(plan::GenericExpr::panic(
            parameter,
            panic(),
        )));
        let uninhabited_tuple = plan::TupleExpr::local_get(
            plan::TupleLocalId(0),
            "tuple".into(),
            vec![ValueType::Parameter(parameter)],
        );
        let diverging_tuple =
            plan::TupleExpr::block(vec![diverging_step.clone()], uninhabited_tuple.clone());
        let inhabited_tuple = plan::TupleExpr::value(
            vec![plan::Expr::int(plan::IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        let custom_shape = plan::CustomValueShape::new(
            custom_name.clone(),
            vec![plan::ValueShape::Parameter(parameter)],
            plan::CustomConstructorRefinement::Exact(0),
        );
        let uninhabited_custom = plan::CustomExpr::local_get(
            plan::CustomLocal::from_shape(plan::CustomLocalId(0), custom_shape),
            "boxed".into(),
        );
        let diverging_custom =
            plan::CustomExpr::block(vec![diverging_step], uninhabited_custom.clone());
        let inhabited_custom = plan::CustomExpr::try_constructor(
            plan::CustomConstructor::new(
                plan::CustomType::new(custom_name, vec![ValueType::Int]),
                "Boxed".into(),
                0,
                vec![plan::CustomConstructorField::new(None, ValueType::Int)],
            ),
            vec![plan::Expr::int(plan::IntExpr::value(1.into()))],
        )
        .expect("one field should construct Boxed(Int)");

        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        assert_eq!(
            flow_outcome(super::super::tuple::tuple_expr(
                &uninhabited_tuple,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::super::tuple::tuple_expr(
                &diverging_tuple,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Diverged,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::super::tuple::tuple_expr(
                &inhabited_tuple,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::super::custom::custom_expr(
                &uninhabited_custom,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::super::custom::custom_expr(
                &diverging_custom,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Diverged,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::super::custom::custom_expr(
                &inhabited_custom,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );
    }

    #[test]
    fn uninhabited_generic_cases_preserve_subject_and_branch_order() {
        let parameter = TypeParameterId(0);
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let local = plan::GenericExpr::local_get(
            plan::GenericLocal::new(plan::GenericLocalId(0), parameter),
            "value".into(),
        );
        let panic = || plan::PanicExpr::panic_at(None, plan::PanicSite::unknown());
        let source_stop = || plan::GenericExpr::panic(parameter, panic());
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        let bool_subject_stop = plan::GenericExpr::bool_case(
            plan::BoolExpr::panic(panic()),
            source_stop(),
            source_stop(),
        )
        .expect("matching generic branches should form a Bool case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&bool_subject_stop, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let bool_false = plan::GenericExpr::bool_case(
            plan::BoolExpr::value(false),
            local.clone(),
            source_stop(),
        )
        .expect("matching generic branches should form a Bool case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&bool_false, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let bool_true =
            plan::GenericExpr::bool_case(plan::BoolExpr::value(true), source_stop(), local.clone())
                .expect("matching generic branches should form a Bool case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&bool_true, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let bool_both = plan::GenericExpr::bool_case(
            plan::BoolExpr::equal(
                plan::Expr::int(plan::IntExpr::value(1.into())),
                plan::Expr::int(plan::IntExpr::value(1.into())),
            ),
            source_stop(),
            source_stop(),
        )
        .expect("matching generic branches should form a Bool case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&bool_both, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let int_subject_stop = plan::GenericExpr::int_case(
            plan::IntExpr::panic(panic()),
            vec![(1.into(), source_stop())],
            source_stop(),
        )
        .expect("matching generic branches should form an Int case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&int_subject_stop, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let int_branch_erased = plan::GenericExpr::int_case(
            plan::IntExpr::value(1.into()),
            vec![(1.into(), local.clone())],
            source_stop(),
        )
        .expect("matching generic branches should form an Int case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&int_branch_erased, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let int_fallback_erased = plan::GenericExpr::int_case(
            plan::IntExpr::value(1.into()),
            vec![(1.into(), source_stop())],
            local.clone(),
        )
        .expect("matching generic branches should form an Int case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&int_fallback_erased, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let string_subject_stop = plan::GenericExpr::string_case(
            plan::StringExpr::panic(panic()),
            vec![("selected".into(), source_stop())],
            source_stop(),
        )
        .expect("matching generic branches should form a String case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&string_subject_stop, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let string_branch_erased = plan::GenericExpr::string_case(
            plan::StringExpr::value("selected".into()),
            vec![("selected".into(), local.clone())],
            source_stop(),
        )
        .expect("matching generic branches should form a String case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&string_branch_erased, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let string_fallback_erased = plan::GenericExpr::string_case(
            plan::StringExpr::value("selected".into()),
            vec![("selected".into(), source_stop())],
            local.clone(),
        )
        .expect("matching generic branches should form a String case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&string_fallback_erased, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let float_subject_stop = plan::GenericExpr::float_case(
            plan::FloatExpr::panic(panic()),
            vec![(1.5, source_stop())],
            source_stop(),
        )
        .expect("matching generic branches should form a Float case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&float_subject_stop, cursor, &mut graph, &mut context),
            Representability::Inhabited(()),
        );

        let float_branch_erased = plan::GenericExpr::float_case(
            plan::FloatExpr::value(1.5),
            vec![(1.5, local.clone())],
            source_stop(),
        )
        .expect("matching generic branches should form a Float case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&float_branch_erased, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );

        let float_fallback_erased = plan::GenericExpr::float_case(
            plan::FloatExpr::value(1.5),
            vec![(1.5, source_stop())],
            local,
        )
        .expect("matching generic branches should form a Float case");
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(&float_fallback_erased, cursor, &mut graph, &mut context),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn stored_generic_operations_stop_at_each_source_owner() {
        let parameter = TypeParameterId(0);
        let boxed_name = plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let boxed_int_shape = plan::CustomValueShape::new(
            boxed_name.clone(),
            vec![ValueShape::Int],
            plan::CustomConstructorRefinement::Exact(0),
        );
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            boxed_definition(boxed_name),
        ]);
        let generic_panic = || plan::GenericExpr::panic(parameter, source_stop());
        let expressions = [
            plan::GenericExpr::call(
                parameter,
                plan::monomorphic_function_instantiation(
                    99,
                    plan::FunctionShape::new(
                        vec![ValueShape::Parameter(parameter)],
                        ValueShape::Parameter(parameter),
                    ),
                ),
                vec![plan::CallArg::new(plan::Expr::generic(generic_panic()))],
            ),
            plan::GenericExpr::function_call(
                plan::GenericFunctionExpr::panic(
                    source_stop(),
                    plan::GenericFunctionType::new(
                        vec![ValueShape::Parameter(parameter)],
                        parameter,
                    ),
                ),
                vec![plan::CallArg::new(plan::Expr::generic(generic_panic()))],
            ),
            plan::GenericExpr::tuple_index(
                parameter,
                plan::TupleExpr::panic(source_stop(), vec![ValueType::Int]),
                0,
            ),
            plan::GenericExpr::custom_field(
                parameter,
                plan::CustomFieldAccess::new(
                    plan::CustomExpr::panic_shape(source_stop(), boxed_int_shape),
                    0,
                    None,
                ),
            ),
            plan::GenericExpr::list_index(
                plan::ListExpr::panic(source_stop(), ValueType::Parameter(parameter))
                    .into_generic()
                    .expect("a parameter item type should create a generic list"),
                0,
            ),
            plan::GenericExpr::block(
                vec![plan::Step::evaluate(plan::Expr::generic(generic_panic()))],
                plan::GenericExpr::local_get(
                    plan::GenericLocal::new(plan::GenericLocalId(0), parameter),
                    "value".into(),
                ),
            ),
        ];
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(stored_expr(
                    &expression,
                    &StoredValueShape::Int,
                    cursor,
                    &mut graph,
                    &mut context,
                )),
                FlowOutcome::Diverged,
            );
        }

        let local_get = plan::GenericExpr::local_get(
            plan::GenericLocal::new(plan::GenericLocalId(0), parameter),
            "value".into(),
        );
        let key = crate::plan::execution::lowering::local::LocalKey::new(
            crate::plan::execution::lowering::local::LocalKind::Generic,
            0,
        );
        let (mut local_graph, cursor) = DraftGraphBuilder::<DraftValueRef, ()>::new(
            vec![(key, StoredValueShape::Int)],
            Vec::new(),
        );
        assert_eq!(
            flow_outcome(stored_expr(
                &local_get,
                &StoredValueShape::Int,
                cursor,
                &mut local_graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        let uninhabited_source = plan::GenericExpr::tuple_index(
            parameter,
            plan::TupleExpr::local_get(
                plan::TupleLocalId(0),
                "tuple".into(),
                vec![ValueType::Parameter(TypeParameterId(1))],
            ),
            0,
        );
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(stored_expr(
                &uninhabited_source,
                &StoredValueShape::Int,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );
    }

    #[test]
    fn uninhabited_generic_projections_and_field_prefixes_preserve_reachability() {
        let parameter = TypeParameterId(0);
        let boxed_name = plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let boxed_int = plan::CustomType::new(boxed_name.clone(), vec![ValueType::Int]);
        let constructor = plan::CustomConstructor::new(
            boxed_int,
            "Boxed".into(),
            0,
            vec![plan::CustomConstructorField::new(None, ValueType::Int)],
        );
        let boxed_value = plan::CustomExpr::try_constructor(
            constructor,
            vec![plan::Expr::int(plan::IntExpr::value(1.into()))],
        )
        .expect("one field should construct Boxed(Int)");
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            boxed_definition(boxed_name),
        ]);
        let tuple_projection = plan::GenericExpr::tuple_index(
            parameter,
            plan::TupleExpr::value(
                vec![plan::Expr::int(plan::IntExpr::value(1.into()))],
                vec![ValueType::Int],
            ),
            0,
        );
        let diverging_tuple_projection = plan::GenericExpr::tuple_index(
            parameter,
            plan::TupleExpr::panic(source_stop(), vec![ValueType::Parameter(parameter)]),
            0,
        );
        let custom_projection = plan::GenericExpr::custom_field(
            parameter,
            plan::CustomFieldAccess::new(boxed_value, 0, None),
        );
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for expression in [tuple_projection, custom_projection] {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                never_expr(&expression, cursor, &mut graph, &mut context),
                Representability::Uninhabited,
            );
        }

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            never_expr(
                &diverging_tuple_projection,
                cursor,
                &mut graph,
                &mut context,
            ),
            Representability::Inhabited(()),
        );

        let inhabited = plan::Expr::int(plan::IntExpr::value(1.into()));
        let uninhabited = plan::Expr::generic(plan::GenericExpr::local_get(
            plan::GenericLocal::new(plan::GenericLocalId(0), parameter),
            "value".into(),
        ));
        let diverged = plan::Expr::int(plan::IntExpr::panic(source_stop()));
        let cases = [
            (vec![inhabited.clone()], 0, Representability::Uninhabited),
            (
                vec![uninhabited, inhabited.clone()],
                1,
                Representability::Uninhabited,
            ),
            (
                vec![diverged, inhabited.clone()],
                1,
                Representability::Inhabited(()),
            ),
            (vec![inhabited], 1, Representability::Uninhabited),
        ];
        for (values, diverging, expected) in cases {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                diverging_values(
                    values.as_slice(),
                    diverging,
                    cursor,
                    &mut graph,
                    &mut context
                ),
                expected,
            );
        }

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            super::uninhabited_expr(
                &plan::Expr::int(plan::IntExpr::value(1.into())),
                &UninhabitedValueShape::Parameter(parameter),
                cursor,
                &mut graph,
                &mut context,
            ),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn uninhabited_tuple_and_custom_projections_stop_or_erase_at_their_source() {
        let parameter = TypeParameterId(0);
        let boxed_name = plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let boxed_parameter_shape = plan::CustomValueShape::new(
            boxed_name.clone(),
            vec![ValueShape::Parameter(parameter)],
            plan::CustomConstructorRefinement::Exact(0),
        );
        let boxed_int = plan::CustomType::new(boxed_name.clone(), vec![ValueType::Int]);
        let boxed_value = plan::CustomExpr::try_constructor(
            plan::CustomConstructor::new(
                boxed_int,
                "Boxed".into(),
                0,
                vec![plan::CustomConstructorField::new(None, ValueType::Int)],
            ),
            vec![plan::Expr::int(plan::IntExpr::value(1.into()))],
        )
        .expect("one field should construct Boxed(Int)");
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            boxed_definition(boxed_name),
        ]);
        let tuple_type = vec![ValueType::Parameter(parameter)];
        let tuple_function_type =
            plan::FunctionType::new(Vec::new(), ValueType::Tuple(tuple_type.clone()));
        let tuple_expressions = [
            plan::TupleExpr::function_call(
                plan::TupleFunctionExpr::panic(source_stop(), tuple_function_type),
                Vec::new(),
                tuple_type.clone(),
            ),
            plan::TupleExpr::tuple_index(
                plan::TupleExpr::value(
                    vec![plan::Expr::int(plan::IntExpr::value(1.into()))],
                    vec![ValueType::Int],
                ),
                0,
                tuple_type.clone(),
            ),
            plan::TupleExpr::custom_field(
                plan::CustomFieldAccess::new(boxed_value.clone(), 0, None),
                tuple_type,
            ),
        ];
        let custom_function_type =
            plan::CustomFunctionType::from_shapes(Vec::new(), boxed_parameter_shape.clone());
        let custom_expressions = [
            plan::CustomExpr::function_call(
                plan::CustomFunctionExpr::panic(source_stop(), custom_function_type),
                Vec::new(),
            ),
            plan::CustomExpr::tuple_index_shape(
                plan::TupleExpr::value(
                    vec![plan::Expr::int(plan::IntExpr::value(1.into()))],
                    vec![ValueType::Int],
                ),
                0,
                boxed_parameter_shape.clone(),
            ),
            plan::CustomExpr::custom_field_shape(
                plan::CustomFieldAccess::new(boxed_value, 0, None),
                boxed_parameter_shape,
            ),
        ];
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for (index, expression) in tuple_expressions.into_iter().enumerate() {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                diverging_values(
                    &[plan::Expr::tuple(expression)],
                    0,
                    cursor,
                    &mut graph,
                    &mut context,
                ),
                if index == 0 {
                    Representability::Inhabited(())
                } else {
                    Representability::Uninhabited
                },
            );
        }
        for (index, expression) in custom_expressions.into_iter().enumerate() {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                diverging_values(
                    &[plan::Expr::custom(expression)],
                    0,
                    cursor,
                    &mut graph,
                    &mut context,
                ),
                if index == 0 {
                    Representability::Inhabited(())
                } else {
                    Representability::Uninhabited
                },
            );
        }
    }

    #[test]
    fn uninhabited_tuple_and_custom_cases_preserve_every_subject_path() {
        let parameter = TypeParameterId(0);
        let boxed_name = plan::CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let boxed_shape = plan::CustomValueShape::new(
            boxed_name.clone(),
            vec![ValueShape::Parameter(parameter)],
            plan::CustomConstructorRefinement::Exact(0),
        );
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            boxed_definition(boxed_name),
        ]);
        let tuple_local = || {
            plan::TupleExpr::local_get(
                plan::TupleLocalId(0),
                "tuple".into(),
                vec![ValueType::Parameter(parameter)],
            )
        };
        let tuple_stop =
            || plan::TupleExpr::panic(source_stop(), vec![ValueType::Parameter(parameter)]);
        let custom_local = || {
            plan::CustomExpr::local_get(
                plan::CustomLocal::from_shape(plan::CustomLocalId(0), boxed_shape.clone()),
                "boxed".into(),
            )
        };
        let custom_stop = || plan::CustomExpr::panic_shape(source_stop(), boxed_shape.clone());
        let tuple_bool_cases = [
            plan::TupleExpr::bool_case(
                plan::BoolExpr::panic(source_stop()),
                tuple_local(),
                tuple_local(),
            ),
            plan::TupleExpr::bool_case(plan::BoolExpr::value(true), tuple_stop(), tuple_local()),
            plan::TupleExpr::bool_case(plan::BoolExpr::value(false), tuple_local(), tuple_stop()),
        ];
        let custom_bool_cases = [
            plan::CustomExpr::bool_case(
                plan::BoolExpr::panic(source_stop()),
                plan::CustomBoolCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    custom_local(),
                    custom_local(),
                ),
            ),
            plan::CustomExpr::bool_case(
                plan::BoolExpr::value(true),
                plan::CustomBoolCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    custom_stop(),
                    custom_local(),
                ),
            ),
            plan::CustomExpr::bool_case(
                plan::BoolExpr::value(false),
                plan::CustomBoolCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    custom_local(),
                    custom_stop(),
                ),
            ),
        ];
        let tuple_switches = [
            plan::TupleExpr::int_case(
                plan::IntExpr::panic(source_stop()),
                vec![(1.into(), tuple_local())],
                tuple_stop(),
            ),
            plan::TupleExpr::int_case(
                plan::IntExpr::value(1.into()),
                vec![(1.into(), tuple_local())],
                tuple_stop(),
            ),
            plan::TupleExpr::string_case(
                plan::StringExpr::panic(source_stop()),
                vec![("one".into(), tuple_local())],
                tuple_stop(),
            ),
            plan::TupleExpr::string_case(
                plan::StringExpr::value("one".into()),
                vec![("one".into(), tuple_local())],
                tuple_stop(),
            ),
            plan::TupleExpr::float_case(
                plan::FloatExpr::panic(source_stop()),
                vec![(1.0, tuple_local())],
                tuple_stop(),
            ),
            plan::TupleExpr::float_case(
                plan::FloatExpr::value(1.0),
                vec![(1.0, tuple_local())],
                tuple_stop(),
            ),
        ];
        let custom_switches = [
            plan::CustomExpr::int_case(
                plan::IntExpr::panic(source_stop()),
                plan::CustomCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    vec![(1.into(), custom_local())],
                    custom_stop(),
                ),
            ),
            plan::CustomExpr::int_case(
                plan::IntExpr::value(1.into()),
                plan::CustomCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    vec![(1.into(), custom_local())],
                    custom_stop(),
                ),
            ),
            plan::CustomExpr::string_case(
                plan::StringExpr::panic(source_stop()),
                plan::CustomCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    vec![("one".into(), custom_local())],
                    custom_stop(),
                ),
            ),
            plan::CustomExpr::string_case(
                plan::StringExpr::value("one".into()),
                plan::CustomCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    vec![("one".into(), custom_local())],
                    custom_stop(),
                ),
            ),
            plan::CustomExpr::float_case(
                plan::FloatExpr::panic(source_stop()),
                plan::CustomCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    vec![(1.0, custom_local())],
                    custom_stop(),
                ),
            ),
            plan::CustomExpr::float_case(
                plan::FloatExpr::value(1.0),
                plan::CustomCaseBranches::from_resolved_shape(
                    boxed_shape.clone(),
                    vec![(1.0, custom_local())],
                    custom_stop(),
                ),
            ),
        ];
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for expression in tuple_bool_cases {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                diverging_values(
                    &[plan::Expr::tuple(expression)],
                    0,
                    cursor,
                    &mut graph,
                    &mut context,
                ),
                Representability::Inhabited(()),
            );
        }
        for expression in custom_bool_cases {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                diverging_values(
                    &[plan::Expr::custom(expression)],
                    0,
                    cursor,
                    &mut graph,
                    &mut context,
                ),
                Representability::Inhabited(()),
            );
        }
        for (index, expression) in tuple_switches.into_iter().enumerate() {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                diverging_values(
                    &[plan::Expr::tuple(expression)],
                    0,
                    cursor,
                    &mut graph,
                    &mut context,
                ),
                if index % 2 == 0 {
                    Representability::Inhabited(())
                } else {
                    Representability::Uninhabited
                },
            );
        }
        for (index, expression) in custom_switches.into_iter().enumerate() {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                diverging_values(
                    &[plan::Expr::custom(expression)],
                    0,
                    cursor,
                    &mut graph,
                    &mut context,
                ),
                if index % 2 == 0 {
                    Representability::Inhabited(())
                } else {
                    Representability::Uninhabited
                },
            );
        }
    }
}
