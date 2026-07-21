use super::{call_args, expr, function, generic, list, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{DraftCursor, DraftCustom, DraftFlow, DraftGraph};
use crate::plan::execution::lowering::specialization::{
    CompoundInhabitation, Representability, SpecializedCustomValueShape, StoredValueShape,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_expr(
    expression: &module::CustomExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftCustom>> {
    let shape = context.concrete_custom_value_shape(expression.shape());
    if let CompoundInhabitation::Uninhabited(proof) =
        context.representations.custom_inhabitation(&shape)
    {
        return generic::custom_never_expr(expression, &proof, cursor, graph, context)
            .map(|_| DraftFlow::Diverged);
    }

    custom_expr_kind(expression.kind(), &shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn custom_expr_kind(
    kind: &module::CustomExprKind,
    shape: &SpecializedCustomValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftCustom>> {
    use super::super::instruction::DraftCustomInstruction as I;
    use module::CustomExprKind as E;

    let stored = StoredValueShape::Custom(shape.clone());
    match kind {
        E::Constructor(construction) => lower_fields(construction.fields(), cursor, graph, context)
            .map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: fields,
                } => {
                    let constructor =
                        context.custom_constructor(construction.constructor().clone());
                    let value = graph.custom_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Construct {
                            constructor,
                            fields,
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            }),
        E::Constant(reference) => context.custom_constant(reference).map(|id| {
            let mut cursor = cursor;
            let value = graph.custom_instruction(
                &mut cursor,
                shape.clone(),
                I::Constant(execution::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, value)
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .custom(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::Custom,
                    local.id().0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Call { function, args } => {
            call_args(args, cursor, graph, context).and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value {
                    mut cursor,
                    value: args,
                } => context.custom_function_id(function, shape).map(|function| {
                    let value = graph.custom_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Call { function, args },
                    );
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall(call) => {
            function::custom_function_expr(call.function(), cursor, graph, context).and_then(
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
                                let value = graph.custom_instruction(
                                    &mut cursor,
                                    shape.clone(),
                                    I::FunctionCall {
                                        function: function.value().clone(),
                                        args,
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
                let value = graph.custom_instruction(
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
            custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.custom_instruction(
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
        } => list::custom_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: list,
            } => {
                let value = graph.custom_instruction(
                    &mut cursor,
                    shape.clone(),
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
            |cursor, graph, context| custom_expr_kind(true_, shape, cursor, graph, context),
            |cursor, graph, context| custom_expr_kind(false_, shape, cursor, graph, context),
            DraftCustom::from_ref,
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
            |branch, cursor, graph, context| {
                custom_expr_kind(branch, shape, cursor, graph, context)
            },
            DraftCustom::from_ref,
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
            |branch, cursor, graph, context| {
                custom_expr_kind(branch, shape, cursor, graph, context)
            },
            DraftCustom::from_ref,
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
            |branch, cursor, graph, context| {
                custom_expr_kind(branch, shape, cursor, graph, context)
            },
            DraftCustom::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    custom_expr_kind(return_, shape, cursor, graph, context)
                }
            }),
    }
}

fn lower_fields(
    fields: &[module::Expr],
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<Vec<super::super::DraftValueRef>>> {
    let mut values = Vec::with_capacity(fields.len());
    for field in fields {
        match expr(field, cursor, graph, context) {
            Representability::Uninhabited => return Representability::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => {
                return Representability::Inhabited(DraftFlow::Diverged);
            }
            Representability::Inhabited(DraftFlow::Value {
                cursor: next,
                value,
            }) => {
                cursor = next;
                values.push(value);
            }
        }
    }
    Representability::Inhabited(DraftFlow::value(cursor, values))
}

#[cfg(test)]
mod tests {
    use super::custom_expr;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftGraphBuilder, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use crate::plan::{
        CustomConstructor, CustomConstructorDefinition, CustomConstructorField,
        CustomFieldDefinition, CustomType, CustomTypeDefinition, CustomTypeName,
        CustomTypePublicity, CustomTypeTemplate, Expr, FunctionShape, FunctionTemplateId, IntExpr,
        PanicExpr, PanicSite, ValueShape, ValueType, monomorphic_function_instantiation,
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
    fn custom_construction_propagates_an_erased_field_specialization() {
        let name = CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let definition = CustomTypeDefinition::new(
            name.clone(),
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(None, CustomTypeTemplate::Int)],
            )],
        );
        let function =
            monomorphic_function_instantiation(0, FunctionShape::new(Vec::new(), ValueShape::Int));
        let constructor = CustomConstructor::new(
            CustomType::new(name, Vec::new()),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let expressions = [
            (IntExpr::value(1.into()), FlowOutcome::Value),
            (
                IntExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
                FlowOutcome::Diverged,
            ),
            (
                IntExpr::call(function, Vec::new()),
                FlowOutcome::Uninhabited,
            ),
        ];
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(vec![definition]);
        context
            .erased_specializations
            .insert(SpecializationKey::monomorphic(FunctionTemplateId::new(0)));
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for (field, expected) in expressions {
            let expression = crate::plan::CustomExpr::try_constructor(
                constructor.clone(),
                vec![Expr::int(field)],
            )
            .expect("one Int field should construct Boxed");
            let cursor = graph.empty_block(cursor.scope().clone());
            assert_eq!(
                flow_outcome(custom_expr(&expression, cursor, &mut graph, &mut context)),
                expected,
            );
        }
    }

    #[test]
    fn custom_fields_preserve_left_to_right_source_stops() {
        for (expression, expected) in [
            (
                "Pair(failed(\"first\"), failed(\"second\"))",
                "panic: first",
            ),
            ("Pair(1, failed(\"second\"))", "panic: second"),
        ] {
            assert_eq!(run(expression), expected);
        }
    }

    fn run(expression: &str) -> String {
        let source = format!(
            r#"
pub type Pair {{ Pair(Int, Int) }}

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
