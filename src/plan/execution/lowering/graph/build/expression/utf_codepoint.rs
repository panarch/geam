use super::{call_args, custom, function, list, tuple};
use crate::plan::execution::lowering::graph::{
    DraftCursor, DraftFlow, DraftGraph, DraftUtfCodepoint,
};
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::module;

pub(in crate::plan::execution::lowering) fn utf_codepoint_expr(
    expression: &module::UtfCodepointExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftUtfCodepoint>> {
    use super::super::instruction::DraftUtfCodepointInstruction as I;
    use module::UtfCodepointExprKind as E;

    match expression.kind() {
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .utf_codepoint(super::super::local::LocalKey::new(
                    super::super::local::LocalKind::UtfCodepoint,
                    local.0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Call { function, args } => {
            call_args(args, cursor, graph, context).and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value {
                    mut cursor,
                    value: args,
                } => context.utf_codepoint_function_id(function).map(|function| {
                    let value =
                        graph.utf_codepoint_instruction(&mut cursor, I::Call { function, args });
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall {
            function: value,
            args,
        } => {
            function::utf_codepoint_function_expr(value, cursor, graph, context).and_then(|flow| {
                match flow {
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
                            let value = graph.utf_codepoint_instruction(
                                &mut cursor,
                                I::FunctionCall {
                                    function: function.value().clone(),
                                    args,
                                },
                            );
                            DraftFlow::value(cursor, value)
                        }
                    }),
                }
            })
        }
        E::TupleIndex {
            tuple: value,
            index,
        } => tuple::tuple_expr(value, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.utf_codepoint_instruction(
                    &mut cursor,
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
                    let value = graph.utf_codepoint_instruction(
                        &mut cursor,
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::ListIndex { list: value, index } => {
            list::utf_codepoint_list_expr(value, cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.utf_codepoint_instruction(
                        &mut cursor,
                        I::ListIndex {
                            list: list.value().clone(),
                            index: *index,
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::Panic(value) => {
            super::panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged)
        }
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::UtfCodepoint),
            |cursor, graph, context| utf_codepoint_expr(true_, cursor, graph, context),
            |cursor, graph, context| utf_codepoint_expr(false_, cursor, graph, context),
            DraftUtfCodepoint::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::UtfCodepoint),
            utf_codepoint_expr,
            DraftUtfCodepoint::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::UtfCodepoint),
            utf_codepoint_expr,
            DraftUtfCodepoint::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::UtfCodepoint),
            utf_codepoint_expr,
            DraftUtfCodepoint::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    utf_codepoint_expr(return_, cursor, graph, context)
                }
            }),
    }
}
