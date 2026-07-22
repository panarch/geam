use super::super::{call_args, custom, list, tuple};
use super::{closure, function_function_expr, reference, source_stop};
use crate::plan::execution::graph::FunctionTarget;
use crate::plan::execution::lowering::graph::{
    DraftCursor, DraftFlow, DraftGraph, DraftListFunction,
};
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn list_function_expr(
    expression: &module::ListFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftListFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::ListFunctionExprKind as E;

    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.type_().clone(),
    ));
    let item_shape = context.concrete_value_shape(&crate::plan::ValueShape::from_value_type(
        expression.return_item_type(),
    ));
    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    match expression.kind() {
        E::Constant(value) => context.list_function_constant(value).map(|id| {
            let mut cursor = cursor;
            let value = graph.function_instruction(
                &mut cursor,
                shape.clone(),
                I::Constant(execution::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, DraftListFunction::new(value))
        }),
        E::Reference(value) => context
            .list_function_id(value.instantiation(), &item_shape)
            .map(|target| {
                reference(shape.clone(), FunctionTarget::List(target), cursor, graph)
                    .map(DraftListFunction::new)
            }),
        E::Closure { function, captures } => context
            .list_function_id(function, &item_shape)
            .and_then(|target| {
                closure(
                    function,
                    captures,
                    shape.clone(),
                    FunctionTarget::List(target),
                    cursor,
                    graph,
                    context,
                )
                .map(|flow| flow.map(DraftListFunction::new))
            }),
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::list_function_local_key(local));
            Representability::Inhabited(DraftFlow::value(cursor, DraftListFunction::new(value)))
        }
        E::Call {
            function,
            args,
            type_: _,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => context
                .list_function_function_id(function, &shape, &item_shape)
                .map(|function| {
                    let function = execution::FunctionFunctionId::List(function);
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Call { function, args },
                    );
                    DraftFlow::value(cursor, DraftListFunction::new(value))
                }),
        }),
        E::FunctionCall {
            function,
            args,
            type_: _,
        } => function_function_expr(function, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                cursor,
                value: function,
            } => call_args(args, cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: args,
                } => {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::FunctionCall {
                            function: function.value().clone(),
                            args,
                        },
                    );
                    DraftFlow::value(cursor, DraftListFunction::new(value))
                }
            }),
        }),
        E::TupleIndex {
            tuple: source,
            index,
            type_: _,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftListFunction::new(value))
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, DraftListFunction::new(value))
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
            type_: _,
        } => list::function_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: list,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftListFunction::new(value))
            }
        }),
        E::Panic(value) => {
            source_stop(value, cursor, graph, context).map(|flow| flow.map(DraftListFunction::new))
        }
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| list_function_expr(true_, cursor, graph, context),
            |cursor, graph, context| list_function_expr(false_, cursor, graph, context),
            DraftListFunction::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            list_function_expr,
            DraftListFunction::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            list_function_expr,
            DraftListFunction::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            list_function_expr,
            DraftListFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        list_function_expr(return_, cursor, graph, context)
                    }
                }
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_list_function_expr(
    expression: &module::GenericFunctionExpr,
    item_shape: &crate::plan::execution::lowering::specialization::SpecializedValueShape,
    shape: &crate::plan::execution::lowering::specialization::SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftListFunction>> {
    generic_list_function_expr_kind(expression.kind(), item_shape, shape, cursor, graph, context)
}

fn generic_list_function_expr_kind(
    kind: &module::GenericFunctionExprKind,
    item_shape: &crate::plan::execution::lowering::specialization::SpecializedValueShape,
    shape: &crate::plan::execution::lowering::specialization::SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftListFunction>> {
    super::generic::lower_executable_kind(
        kind,
        shape,
        cursor,
        graph,
        context,
        super::generic::executable_kind_lowering(
            DraftListFunction::new,
            |value, context| {
                context
                    .generic_list_function_constant(value, item_shape, shape)
                    .map(|id| id.index())
            },
            |function, context| {
                context
                    .list_function_id(function, item_shape)
                    .map(FunctionTarget::List)
            },
            |function, context| {
                context
                    .list_function_function_id(function, shape, item_shape)
                    .map(execution::FunctionFunctionId::List)
            },
            |branch, cursor, graph, context| {
                generic_list_function_expr_kind(branch, item_shape, shape, cursor, graph, context)
            },
        ),
    )
}
