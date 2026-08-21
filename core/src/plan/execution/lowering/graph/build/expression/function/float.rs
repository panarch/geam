use super::super::{call_args, custom, list, tuple};
use super::{closure, function_function_expr, reference, source_stop};
use crate::plan::execution::lowering::graph::DraftFunctionTarget;
use crate::plan::execution::lowering::graph::{
    DraftCursor, DraftFloatFunction, DraftFlow, DraftGraph,
};
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn float_function_expr(
    expression: &module::FloatFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFloatFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::FloatFunctionExprKind as E;

    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.type_().clone(),
    ));
    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    match expression.kind() {
        E::Constant(value) => context.float_function_constant(value).map(|id| {
            let mut cursor = cursor;
            let value = graph.function_instruction(
                &mut cursor,
                shape.clone(),
                I::Constant(execution::constant::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, DraftFloatFunction::new(value))
        }),
        E::Reference(value) => context
            .float_function_id(value.instantiation())
            .map(|target| {
                reference(
                    shape.clone(),
                    DraftFunctionTarget::Float(target),
                    cursor,
                    graph,
                )
                .map(DraftFloatFunction::new)
            }),
        E::Closure { function, captures } => {
            context.float_function_id(function).and_then(|target| {
                closure(
                    function,
                    captures,
                    shape.clone(),
                    DraftFunctionTarget::Float(target),
                    cursor,
                    graph,
                    context,
                )
                .map(|flow| flow.map(DraftFloatFunction::new))
            })
        }
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::FloatFunction,
                    local.0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, DraftFloatFunction::new(value)))
        }
        E::Call {
            function,
            args,
            type_: _,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => context
                .float_function_function_id(function)
                .map(|function| {
                    let function = execution::function::FunctionFunctionId::Float(function);
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Call {
                            function,
                            args,
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, DraftFloatFunction::new(value))
                }),
        }),
        E::FunctionCall {
            function,
            args,
            type_: _,
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
                    DraftFlow::value(cursor, DraftFloatFunction::new(value))
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
                DraftFlow::value(cursor, DraftFloatFunction::new(value))
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
                    DraftFlow::value(cursor, DraftFloatFunction::new(value))
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
                DraftFlow::value(cursor, DraftFloatFunction::new(value))
            }
        }),
        E::Panic(value) => {
            source_stop(value, cursor, graph, context).map(|flow| flow.map(DraftFloatFunction::new))
        }
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| float_function_expr(true_, cursor, graph, context),
            |cursor, graph, context| float_function_expr(false_, cursor, graph, context),
            DraftFloatFunction::from_ref,
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
            float_function_expr,
            DraftFloatFunction::from_ref,
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
            float_function_expr,
            DraftFloatFunction::from_ref,
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
            float_function_expr,
            DraftFloatFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        float_function_expr(return_, cursor, graph, context)
                    }
                }
            })
        }
    }
}
