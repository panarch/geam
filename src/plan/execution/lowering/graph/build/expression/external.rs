use super::{call_args, custom, function, list, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{DraftCursor, DraftExternal, DraftFlow, DraftGraph};
use crate::plan::execution::lowering::specialization::{
    Representability, SpecializedExternalValueShape, StoredValueShape,
};
use crate::plan::module;

pub(in crate::plan::execution::lowering) fn external_expr(
    expression: &module::ExternalExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftExternal>> {
    let shape = context.concrete_external_value_shape(expression.shape());
    external_expr_kind(expression.kind(), &shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn external_expr_kind(
    kind: &module::ExternalExprKind,
    shape: &SpecializedExternalValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftExternal>> {
    use super::super::instruction::DraftExternalInstruction as I;
    use module::ExternalExprKind as E;

    let stored = StoredValueShape::External(shape.clone());
    match kind {
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().external(super::super::local::LocalKey::new(
                super::super::local::LocalKind::External,
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
                mut cursor,
                value: args,
            } => context
                .external_function_id(function, shape)
                .map(|function| {
                    let value = graph.external_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Call {
                            function,
                            args,
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, value)
                }),
        }),
        E::FunctionCall(call) => {
            function::external_function_expr(call.function(), cursor, graph, context).and_then(
                |flow| match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value {
                        cursor,
                        value: function,
                    } => {
                        call_args(call.arguments(), cursor, graph, context).map(|flow| match flow {
                            DraftFlow::Diverged => DraftFlow::Diverged,
                            DraftFlow::Value {
                                mut cursor,
                                value: args,
                            } => {
                                let value = graph.external_instruction(
                                    &mut cursor,
                                    shape.clone(),
                                    I::FunctionCall {
                                        function: function.value().clone(),
                                        args,
                                        site: call.site().clone(),
                                    },
                                );
                                DraftFlow::value(cursor, value)
                            }
                        })
                    }
                },
            )
        }
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.external_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, value)
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.external_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::external_list_expr(source, cursor, graph, context).map(|flow| {
            flow.map_cursor(|cursor, list| {
                graph.external_instruction(
                    cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                )
            })
        }),
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, stored),
            |cursor, graph, context| external_expr(true_, cursor, graph, context),
            |cursor, graph, context| external_expr(false_, cursor, graph, context),
            DraftExternal::from_ref,
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
            super::case_lowering(graph, context, stored),
            external_expr,
            DraftExternal::from_ref,
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
            super::case_lowering(graph, context, stored),
            external_expr,
            DraftExternal::from_ref,
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
            super::case_lowering(graph, context, stored),
            external_expr,
            DraftExternal::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    external_expr(return_, cursor, graph, context)
                }
            }),
    }
}
