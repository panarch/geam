use super::super::{call_args, custom, list, tuple};
use super::{closure, function_function_expr, reference, source_stop};
use crate::plan::execution::lowering::graph::DraftFunctionTarget;
use crate::plan::execution::lowering::graph::{
    DraftCursor, DraftExternalFunction, DraftFlow, DraftFunction, DraftGraph,
};
use crate::plan::execution::lowering::specialization::{
    Representability, SpecializedExternalValueShape, SpecializedFunctionShape, StoredValueShape,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn external_function_expr(
    expression: &module::ExternalFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftExternalFunction>> {
    let return_shape =
        context.concrete_external_value_shape(expression.external_function_type().return_());
    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.external_function_type().to_function_type(),
    ));
    external_function_expr_kind(
        expression.kind(),
        &return_shape,
        &shape,
        cursor,
        graph,
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_external_function_expr(
    expression: &module::GenericFunctionExpr,
    return_shape: &SpecializedExternalValueShape,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    generic_external_function_expr_kind(
        expression.kind(),
        return_shape,
        shape,
        cursor,
        graph,
        context,
    )
}

fn generic_external_function_expr_kind(
    kind: &module::GenericFunctionExprKind,
    return_shape: &SpecializedExternalValueShape,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    super::generic::lower_executable_kind(
        kind,
        shape,
        cursor,
        graph,
        context,
        super::generic::executable_kind_lowering(
            std::convert::identity,
            |value, context| {
                context
                    .generic_external_function_constant(value, return_shape, shape)
                    .map(|id| id.index())
            },
            |function, context| {
                context
                    .external_function_id(function, return_shape)
                    .map(DraftFunctionTarget::External)
            },
            |function, context| {
                let type_ =
                    context.specialized_external_function_type(shape.arguments(), return_shape);
                context
                    .external_function_function_id(function, type_)
                    .map(execution::function::FunctionFunctionId::External)
            },
            |branch, cursor, graph, context| {
                generic_external_function_expr_kind(
                    branch,
                    return_shape,
                    shape,
                    cursor,
                    graph,
                    context,
                )
            },
        ),
    )
}

pub(in crate::plan::execution::lowering) fn external_function_expr_kind(
    kind: &module::ExternalFunctionExprKind,
    return_shape: &SpecializedExternalValueShape,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftExternalFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::ExternalFunctionExprKind as E;

    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    let type_ = context.specialized_external_function_type(shape.arguments(), return_shape);
    match kind {
        E::Constant(value) => context.external_function_constant(value).map(|id| {
            let mut cursor = cursor;
            let value = graph.function_instruction(
                &mut cursor,
                shape.clone(),
                I::Constant(execution::constant::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, DraftExternalFunction::new(value))
        }),
        E::Reference(value) => context
            .external_function_id(value.instantiation(), return_shape)
            .map(|target| {
                reference(
                    shape.clone(),
                    DraftFunctionTarget::External(target),
                    cursor,
                    graph,
                )
                .map(DraftExternalFunction::new)
            }),
        E::Closure { function, captures } => context
            .external_function_id(function, return_shape)
            .and_then(|target| {
                closure(
                    function,
                    captures,
                    shape.clone(),
                    DraftFunctionTarget::External(target),
                    cursor,
                    graph,
                    context,
                )
                .map(|flow| flow.map(DraftExternalFunction::new))
            }),
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::ExternalFunction,
                    local.id().0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, DraftExternalFunction::new(value)))
        }
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => context
                .external_function_function_id(function, type_.clone())
                .map(|function| {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Call {
                            function: execution::function::FunctionFunctionId::External(function),
                            args,
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, DraftExternalFunction::new(value))
                }),
        }),
        E::FunctionCall {
            function,
            args,
            site,
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
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, DraftExternalFunction::new(value))
                }
            }),
        }),
        E::TupleIndex {
            tuple: source,
            index,
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
                DraftFlow::value(cursor, DraftExternalFunction::new(value))
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
                    DraftFlow::value(cursor, DraftExternalFunction::new(value))
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::function_list_expr(source, cursor, graph, context).map(|flow| {
            flow.map_cursor(|cursor, list| {
                let value = graph.function_instruction(
                    cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                DraftExternalFunction::new(value)
            })
        }),
        E::Panic(value) => source_stop(value, cursor, graph, context)
            .map(|flow| flow.map(DraftExternalFunction::new)),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| {
                external_function_expr_kind(true_, return_shape, shape, cursor, graph, context)
            },
            |cursor, graph, context| {
                external_function_expr_kind(false_, return_shape, shape, cursor, graph, context)
            },
            DraftExternalFunction::from_ref,
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
            |branch, cursor, graph, context| {
                external_function_expr_kind(branch, return_shape, shape, cursor, graph, context)
            },
            DraftExternalFunction::from_ref,
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
            |branch, cursor, graph, context| {
                external_function_expr_kind(branch, return_shape, shape, cursor, graph, context)
            },
            DraftExternalFunction::from_ref,
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
            |branch, cursor, graph, context| {
                external_function_expr_kind(branch, return_shape, shape, cursor, graph, context)
            },
            DraftExternalFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => external_function_expr_kind(
                        return_,
                        return_shape,
                        shape,
                        cursor,
                        graph,
                        context,
                    ),
                }
            })
        }
    }
}
