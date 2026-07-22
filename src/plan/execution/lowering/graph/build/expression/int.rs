use super::{call_args, custom, function, list, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{DraftCursor, DraftFlow, DraftGraph, DraftInt};
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn int_expr(
    expression: &module::IntExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftInt>> {
    use super::super::instruction::DraftIntInstruction as I;
    use module::IntExprKind as E;

    match expression.kind() {
        E::Value(value) => {
            let mut cursor = cursor;
            let value = graph.int_instruction(&mut cursor, I::Value(value.clone()));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Constant(reference) => context.int_constant(reference).map(|id| {
            let mut cursor = cursor;
            let value = graph.int_instruction(
                &mut cursor,
                I::Constant(execution::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, value)
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().int(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Int,
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
                } => context.int_function_id(function).map(|function| {
                    let value = graph.int_instruction(&mut cursor, I::Call { function, args });
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall {
            function: value,
            args,
        } => {
            function::int_function_expr(value, cursor, graph, context).and_then(|flow| match flow {
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
                        let value = graph.int_instruction(
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
                let value = graph.int_instruction(
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
                    let value = graph.int_instruction(
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
        E::ListIndex { list: value, index } => list::int_list_expr(value, cursor, graph, context)
            .map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.int_instruction(
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
        E::Add { left, right } => binary(left, right, cursor, graph, context, |left, right| {
            I::Add { left, right }
        }),
        E::Sub { left, right } => binary(left, right, cursor, graph, context, |left, right| {
            I::Sub { left, right }
        }),
        E::Mult { left, right } => binary(left, right, cursor, graph, context, |left, right| {
            I::Mult { left, right }
        }),
        E::Div { left, right } => binary(left, right, cursor, graph, context, |left, right| {
            I::Div { left, right }
        }),
        E::Remainder { left, right } => {
            binary(left, right, cursor, graph, context, |left, right| {
                I::Remainder { left, right }
            })
        }
        E::Negate(value) => int_expr(value, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value { mut cursor, value } => {
                let value = graph.int_instruction(&mut cursor, I::Negate(value));
                DraftFlow::value(cursor, value)
            }
        }),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::Int),
            |cursor, graph, context| int_expr(true_, cursor, graph, context),
            |cursor, graph, context| int_expr(false_, cursor, graph, context),
            DraftInt::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Int),
            int_expr,
            DraftInt::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Int),
            int_expr,
            DraftInt::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Int),
            int_expr,
            DraftInt::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => int_expr(return_, cursor, graph, context),
            }),
    }
}

fn binary(
    left: &module::IntExpr,
    right: &module::IntExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    kind: impl FnOnce(DraftInt, DraftInt) -> super::super::instruction::DraftIntInstruction,
) -> Representability<DraftFlow<DraftInt>> {
    int_expr(left, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        DraftFlow::Value {
            cursor,
            value: left,
        } => int_expr(right, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: right,
            } => {
                let value = graph.int_instruction(&mut cursor, kind(left, right));
                DraftFlow::value(cursor, value)
            }
        }),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn int_operators_preserve_left_to_right_source_stops() {
        for (expression, expected) in [
            ("failed(\"left\") + failed(\"right\")", "panic: left"),
            ("1 + failed(\"right\")", "panic: right"),
            ("-failed(\"operand\")", "panic: operand"),
            (
                "{ panic as \"left\" } + { panic as \"right\" }",
                "panic: left",
            ),
            ("1 + { panic as \"right\" }", "panic: right"),
            ("-{ panic as \"operand\" }", "panic: operand"),
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
        crate::run_main(&crate::ExecutionPlan::from_module_plan(module))
            .unwrap_err()
            .to_string()
    }
}
