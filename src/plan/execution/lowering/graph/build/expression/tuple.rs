use super::{call_args, custom, expr, function, generic, list, panic_expr};
use crate::plan::execution::lowering::graph::{DraftCursor, DraftFlow, DraftGraph, DraftTuple};
use crate::plan::execution::lowering::specialization::{
    CompoundInhabitation, Representability, StoredValueShape,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn tuple_expr(
    expression: &module::TupleExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftTuple>> {
    use super::super::instruction::DraftTupleInstruction as I;
    use module::TupleExprKind as E;

    let elements = expression
        .shape()
        .iter()
        .map(|shape| context.concrete_value_shape(shape))
        .collect::<Vec<_>>();
    if let CompoundInhabitation::Uninhabited(proof) =
        context.representations.tuple_inhabitation(&elements)
    {
        return generic::tuple_never_expr(expression, &proof, cursor, graph, context)
            .map(|_| DraftFlow::Diverged);
    }
    let stored = StoredValueShape::Tuple(elements.clone().into_boxed_slice());

    match expression.kind() {
        E::Value(values) => lower_values(values, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: values,
            } => {
                let value = graph.tuple_instruction(
                    &mut cursor,
                    elements.clone().into_boxed_slice(),
                    I::Value(values),
                );
                DraftFlow::value(cursor, value)
            }
        }),
        E::Constant(reference) => context.tuple_constant(reference).map(|id| {
            let mut cursor = cursor;
            let value = graph.tuple_instruction(
                &mut cursor,
                elements.clone().into_boxed_slice(),
                I::Constant(execution::constant::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, value)
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().tuple(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Tuple,
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
                } => context.tuple_function_id(function).map(|function| {
                    let value = graph.tuple_instruction(
                        &mut cursor,
                        elements.clone().into_boxed_slice(),
                        I::Call { function, args },
                    );
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall {
            function: value,
            args,
        } => {
            function::tuple_function_expr(value, cursor, graph, context).and_then(|flow| match flow
            {
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
                        let value = graph.tuple_instruction(
                            &mut cursor,
                            elements.clone().into_boxed_slice(),
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
        } => tuple_expr(value, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.tuple_instruction(
                    &mut cursor,
                    elements.clone().into_boxed_slice(),
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
                    let value = graph.tuple_instruction(
                        &mut cursor,
                        elements.clone().into_boxed_slice(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::ListIndex { list: value, index } => list::tuple_list_expr(value, cursor, graph, context)
            .map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.tuple_instruction(
                        &mut cursor,
                        elements.clone().into_boxed_slice(),
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
            super::case_lowering(graph, context, stored),
            |cursor, graph, context| tuple_expr(true_, cursor, graph, context),
            |cursor, graph, context| tuple_expr(false_, cursor, graph, context),
            DraftTuple::from_ref,
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
            tuple_expr,
            DraftTuple::from_ref,
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
            tuple_expr,
            DraftTuple::from_ref,
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
            tuple_expr,
            DraftTuple::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    tuple_expr(return_, cursor, graph, context)
                }
            }),
    }
}

fn lower_values(
    values: &[module::Expr],
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<Vec<super::super::DraftValueRef>>> {
    let mut lowered = Vec::with_capacity(values.len());
    for value in values {
        match expr(value, cursor, graph, context) {
            Representability::Uninhabited => return Representability::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => {
                return Representability::Inhabited(DraftFlow::Diverged);
            }
            Representability::Inhabited(DraftFlow::Value {
                cursor: next,
                value,
            }) => {
                cursor = next;
                lowered.push(value);
            }
        }
    }
    Representability::Inhabited(DraftFlow::value(cursor, lowered))
}

#[cfg(test)]
mod tests {
    use super::tuple_expr;
    use crate::plan::execution::lowering::graph::draft::DraftGraphBuilder;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use crate::plan::{
        Expr, FunctionShape, FunctionTemplateId, IntExpr, PanicExpr, PanicSite, TupleExpr,
        ValueShape, ValueType, monomorphic_function_instantiation,
    };

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

    #[test]
    fn tuple_construction_propagates_an_erased_element_specialization() {
        let instantiation =
            monomorphic_function_instantiation(0, FunctionShape::new(Vec::new(), ValueShape::Int));
        let expressions = [
            (
                TupleExpr::value(
                    vec![Expr::int(IntExpr::value(1.into()))],
                    vec![ValueType::Int],
                ),
                FlowOutcome::Value,
            ),
            (
                TupleExpr::value(
                    vec![Expr::int(IntExpr::panic(PanicExpr::panic_at(
                        None,
                        PanicSite::unknown(),
                    )))],
                    vec![ValueType::Int],
                ),
                FlowOutcome::Diverged,
            ),
            (
                TupleExpr::value(
                    vec![Expr::int(IntExpr::call(instantiation, Vec::new()))],
                    vec![ValueType::Int],
                ),
                FlowOutcome::Uninhabited,
            ),
        ];
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        context
            .erased_specializations
            .insert(SpecializationKey::monomorphic(FunctionTemplateId::new(0)));
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for (expression, expected) in expressions {
            let cursor = graph.empty_block(cursor.scope().clone());
            assert_eq!(
                flow_outcome(tuple_expr(&expression, cursor, &mut graph, &mut context)),
                expected,
            );
        }
    }

    #[test]
    fn tuple_elements_preserve_left_to_right_source_stops() {
        for (expression, expected) in [
            ("#(failed(\"first\"), failed(\"second\"))", "panic: first"),
            ("#(1, failed(\"second\"))", "panic: second"),
            ("#(panic as \"first\", panic as \"second\")", "panic: first"),
            ("#(1, panic as \"second\")", "panic: second"),
        ] {
            assert_eq!(run(expression), expected);
        }
    }

    fn run(expression: &str) -> String {
        let source = format!(
            r#"
fn failed(message: String) -> Int {{ panic as message }}

pub fn main() {{ {expression} }}
"#,
        );
        let typed = crate::compile_typed_module("main", "main.gleam", source.as_str())
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        crate::run_main(
            &crate::ExecutionPlan::from_module_plan(module),
            &mut Vec::new(),
        )
        .unwrap_err()
        .to_string()
    }
}
