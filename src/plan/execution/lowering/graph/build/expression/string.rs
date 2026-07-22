use super::{call_args, custom, function, list, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{DraftCursor, DraftFlow, DraftGraph, DraftString};
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn string_expr(
    expression: &module::StringExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftString>> {
    use super::super::instruction::DraftStringInstruction as I;
    use module::StringExprKind as E;

    match expression.kind() {
        E::Value(value) => {
            let mut cursor = cursor;
            let value = graph.string_instruction(&mut cursor, I::Value(value.clone()));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Constant(reference) => context.string_constant(reference).map(|id| {
            let mut cursor = cursor;
            let value = graph.string_instruction(
                &mut cursor,
                I::Constant(execution::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, value)
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().string(super::super::local::LocalKey::new(
                super::super::local::LocalKind::String,
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
                } => context.string_function_id(function).map(|function| {
                    let value = graph.string_instruction(&mut cursor, I::Call { function, args });
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall {
            function: value,
            args,
        } => {
            function::string_function_expr(value, cursor, graph, context).and_then(
                |flow| match flow {
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
                            let value = graph.string_instruction(
                                &mut cursor,
                                I::FunctionCall {
                                    function: function.value().clone(),
                                    args,
                                },
                            );
                            DraftFlow::value(cursor, value)
                        }
                    }),
                },
            )
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
                let value = graph.string_instruction(
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
                    let value = graph.string_instruction(
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
            list::string_list_expr(value, cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.string_instruction(
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
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::Concatenate { left, right } => {
            string_expr(left, cursor, graph, context).and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value {
                    cursor,
                    value: left,
                } => string_expr(right, cursor, graph, context).map(|flow| match flow {
                    DraftFlow::Diverged => DraftFlow::Diverged,
                    DraftFlow::Value {
                        mut cursor,
                        value: right,
                    } => {
                        let value =
                            graph.string_instruction(&mut cursor, I::Concatenate { left, right });
                        DraftFlow::value(cursor, value)
                    }
                }),
            })
        }
        E::DropPrefix { value, prefix } => {
            string_expr(value, cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value { mut cursor, value } => {
                    let value = graph.string_instruction(
                        &mut cursor,
                        I::DropPrefix {
                            value,
                            prefix: prefix.clone(),
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::String),
            |cursor, graph, context| string_expr(true_, cursor, graph, context),
            |cursor, graph, context| string_expr(false_, cursor, graph, context),
            DraftString::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::String),
            string_expr,
            DraftString::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::String),
            string_expr,
            DraftString::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::String),
            string_expr,
            DraftString::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    string_expr(return_, cursor, graph, context)
                }
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::string_expr;
    use crate::plan::execution::lowering::graph::draft::DraftGraphBuilder;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftValueRef};
    use crate::plan::execution::lowering::specialization::Representability;
    use crate::plan::{
        Expr, GenericExpr, GenericLocal, GenericLocalId, PanicExpr, PanicSite, Step, StringExpr,
        TypeParameterId,
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
    fn string_lowering_preserves_each_terminal_source_outcome() {
        let parameter = TypeParameterId(0);
        let expressions = [
            (StringExpr::value("value".into()), FlowOutcome::Value),
            (
                StringExpr::drop_prefix(
                    StringExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
                    "prefix".into(),
                ),
                FlowOutcome::Diverged,
            ),
            (
                StringExpr::block(
                    vec![Step::evaluate(Expr::generic(GenericExpr::local_get(
                        GenericLocal::new(GenericLocalId(0), parameter),
                        "value".into(),
                    )))],
                    StringExpr::value("unreachable".into()),
                ),
                FlowOutcome::Uninhabited,
            ),
        ];
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for (expression, expected) in expressions {
            let cursor = graph.empty_block(cursor.scope().clone());
            assert_eq!(
                flow_outcome(string_expr(&expression, cursor, &mut graph, &mut context)),
                expected,
            );
        }
    }

    #[test]
    fn string_concatenation_preserves_left_to_right_source_stops() {
        for (expression, expected) in [
            ("failed(\"left\") <> failed(\"right\")", "panic: left"),
            ("\"left\" <> failed(\"right\")", "panic: right"),
            (
                "{ panic as \"left\" } <> { panic as \"right\" }",
                "panic: left",
            ),
            ("\"left\" <> { panic as \"right\" }", "panic: right"),
        ] {
            assert_eq!(run(expression), expected);
        }
    }

    fn run(expression: &str) -> String {
        let source = format!(
            r#"
fn failed(message: String) -> String {{ panic as message }}

pub fn main() {{ {expression} }}
"#,
        );
        let typed = crate::compile_typed_module("main", "main.gleam", source.as_str())
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        crate::run_main(&crate::ExecutionPlan::from_module_plan(module))
            .unwrap_err()
            .to_string()
    }
}
