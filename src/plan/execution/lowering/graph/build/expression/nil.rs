use super::{call_args, custom, function, list, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{DraftCursor, DraftFlow, DraftGraph, DraftNil};
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn nil_expr(
    expression: &module::NilExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftNil>> {
    use super::super::instruction::DraftNilInstruction as I;
    use module::NilExprKind as E;

    match expression.kind() {
        E::Value => {
            let mut cursor = cursor;
            let value = graph.nil_instruction(&mut cursor, I::Value);
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Constant(reference) => context.nil_constant(reference).map(|id| {
            let mut cursor = cursor;
            let value = graph.nil_instruction(
                &mut cursor,
                I::Constant(execution::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, value)
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().nil(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Nil,
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
                } => context.nil_function_id(function).map(|function| {
                    let value = graph.nil_instruction(&mut cursor, I::Call { function, args });
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall {
            function: value,
            args,
        } => {
            function::nil_function_expr(value, cursor, graph, context).and_then(|flow| match flow {
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
                        let value = graph.nil_instruction(
                            &mut cursor,
                            I::FunctionCall {
                                function: function.value().clone(),
                                args,
                            },
                        );
                        DraftFlow::value(cursor, value)
                    }
                }),
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
                let value = graph.nil_instruction(
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
                    let value = graph.nil_instruction(
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
        E::ListIndex { list: value, index } => list::nil_list_expr(value, cursor, graph, context)
            .map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.nil_instruction(
                        &mut cursor,
                        I::ListIndex {
                            list: list.value().clone(),
                            index: *index,
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            }),
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::Nil),
            |cursor, graph, context| nil_expr(true_, cursor, graph, context),
            |cursor, graph, context| nil_expr(false_, cursor, graph, context),
            DraftNil::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Nil),
            nil_expr,
            DraftNil::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Nil),
            nil_expr,
            DraftNil::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Nil),
            nil_expr,
            DraftNil::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => nil_expr(return_, cursor, graph, context),
            }),
    }
}
