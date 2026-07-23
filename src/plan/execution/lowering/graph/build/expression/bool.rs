use super::{call_args, custom, expr, function, list, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{DraftBool, DraftCursor, DraftFlow, DraftGraph};
use crate::plan::execution::lowering::specialization::{
    Representability, SpecializedValueShape, StoredValueShape,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) enum BoolPaths {
    Diverged,
    True(DraftCursor),
    False(DraftCursor),
    Both {
        true_: DraftCursor,
        false_: DraftCursor,
    },
}

pub(in crate::plan::execution::lowering) fn bool_paths(
    expression: &module::BoolExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<BoolPaths> {
    use module::BoolExprKind as E;

    match expression.kind() {
        E::Value(true) => Representability::Inhabited(BoolPaths::True(cursor)),
        E::Value(false) => Representability::Inhabited(BoolPaths::False(cursor)),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(BoolPaths::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    bool_paths(return_, cursor, graph, context)
                }
            }),
        E::And { left, right } => {
            bool_paths(left, cursor, graph, context).and_then(|left| match left {
                BoolPaths::Diverged => Representability::Inhabited(BoolPaths::Diverged),
                BoolPaths::False(cursor) => Representability::Inhabited(BoolPaths::False(cursor)),
                BoolPaths::True(cursor) => bool_paths(right, cursor, graph, context),
                BoolPaths::Both { true_, false_ } => bool_paths(right, true_, graph, context)
                    .map(|right| merge_and(right, false_, graph)),
            })
        }
        E::Or { left, right } => {
            bool_paths(left, cursor, graph, context).and_then(|left| match left {
                BoolPaths::Diverged => Representability::Inhabited(BoolPaths::Diverged),
                BoolPaths::True(cursor) => Representability::Inhabited(BoolPaths::True(cursor)),
                BoolPaths::False(cursor) => bool_paths(right, cursor, graph, context),
                BoolPaths::Both { true_, false_ } => bool_paths(right, false_, graph, context)
                    .map(|right| merge_or(true_, right, graph)),
            })
        }
        E::BitArrayMatches { value, pattern } => {
            super::super::step::bit_array_match_paths(value, pattern, cursor, graph, context)
        }
        E::CustomMatches { value, pattern } => {
            super::super::step::custom_match_paths(value, pattern, cursor, graph, context)
        }
        E::ListLengthEquals { value, length } => {
            list_length_paths(value, *length, true, cursor, graph, context)
        }
        E::ListLengthAtLeast { value, length } => {
            list_length_paths(value, *length, false, cursor, graph, context)
        }
        _ => bool_value_instruction(expression, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => BoolPaths::Diverged,
            DraftFlow::Value { cursor, value } => {
                let scope = cursor.scope().clone();
                let true_ = graph.empty_block(scope.clone());
                let false_ = graph.empty_block(scope);
                let true_id = true_.id();
                let false_id = false_.id();
                graph.finish_bool_branch(cursor, value, true_id, false_id);
                BoolPaths::Both { true_, false_ }
            }
        }),
    }
}

pub(in crate::plan::execution::lowering) fn bool_expr(
    expression: &module::BoolExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftBool>> {
    bool_value_instruction(expression, cursor, graph, context)
}

fn materialize_paths(paths: BoolPaths, graph: &mut DraftGraph) -> DraftFlow<DraftBool> {
    use super::super::instruction::DraftBoolInstruction as I;

    match paths {
        BoolPaths::Diverged => DraftFlow::Diverged,
        BoolPaths::True(mut cursor) => {
            let value = graph.bool_instruction(&mut cursor, I::Value(true));
            DraftFlow::value(cursor, value)
        }
        BoolPaths::False(mut cursor) => {
            let value = graph.bool_instruction(&mut cursor, I::Value(false));
            DraftFlow::value(cursor, value)
        }
        BoolPaths::Both {
            mut true_,
            mut false_,
        } => {
            let scope = true_.scope().clone();
            let true_value = graph.bool_instruction(&mut true_, I::Value(true));
            let false_value = graph.bool_instruction(&mut false_, I::Value(false));
            super::join_branches(
                scope,
                StoredValueShape::Bool,
                vec![
                    DraftFlow::value(true_, true_value),
                    DraftFlow::value(false_, false_value),
                ],
                graph,
                DraftBool::from_ref,
            )
        }
    }
}

fn bool_value_instruction(
    expression: &module::BoolExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<DraftBool>> {
    use super::super::instruction::DraftBoolInstruction as I;
    use module::BoolExprKind as E;

    match expression.kind() {
        E::Value(value) => {
            let mut cursor = cursor;
            let value = graph.bool_instruction(&mut cursor, I::Value(*value));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Constant(reference) => context.bool_constant(reference).map(|id| {
            let mut cursor = cursor;
            let value = graph.bool_instruction(
                &mut cursor,
                I::Constant(execution::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, value)
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor.scope().bool(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Bool,
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
                } => context.bool_function_id(function).map(|function| {
                    let value = graph.bool_instruction(&mut cursor, I::Call { function, args });
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall {
            function: value,
            args,
        } => function::bool_function_expr(value, cursor, graph, context).and_then(
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
                        let value = graph.bool_instruction(
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
        ),
        E::TupleIndex {
            tuple: value,
            index,
        } => tuple::tuple_expr(value, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.bool_instruction(
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
                    let value = graph.bool_instruction(
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
        E::ListIndex { list: value, index } => list::bool_list_expr(value, cursor, graph, context)
            .map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: list,
                } => {
                    let value = graph.bool_instruction(
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
        E::Not(value) => bool_expr(value, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value { mut cursor, value } => {
                let value = graph.bool_instruction(&mut cursor, I::Not(value));
                DraftFlow::value(cursor, value)
            }
        }),
        E::LtInt { left, right } => {
            compare_int(left, right, cursor, graph, context, |left, right| {
                I::LtInt { left, right }
            })
        }
        E::LtEqInt { left, right } => {
            compare_int(left, right, cursor, graph, context, |left, right| {
                I::LtEqInt { left, right }
            })
        }
        E::GtInt { left, right } => {
            compare_int(left, right, cursor, graph, context, |left, right| {
                I::GtInt { left, right }
            })
        }
        E::GtEqInt { left, right } => {
            compare_int(left, right, cursor, graph, context, |left, right| {
                I::GtEqInt { left, right }
            })
        }
        E::LtFloat { left, right } => {
            compare_float(left, right, cursor, graph, context, |left, right| {
                I::LtFloat { left, right }
            })
        }
        E::LtEqFloat { left, right } => {
            compare_float(left, right, cursor, graph, context, |left, right| {
                I::LtEqFloat { left, right }
            })
        }
        E::GtFloat { left, right } => {
            compare_float(left, right, cursor, graph, context, |left, right| {
                I::GtFloat { left, right }
            })
        }
        E::GtEqFloat { left, right } => {
            compare_float(left, right, cursor, graph, context, |left, right| {
                I::GtEqFloat { left, right }
            })
        }
        E::Equal { left, right } => {
            compare_values(left, right, cursor, graph, context, |left, right| {
                I::Equal { left, right }
            })
        }
        E::NotEqual { left, right } => {
            compare_values(left, right, cursor, graph, context, |left, right| {
                I::NotEqual { left, right }
            })
        }
        E::StringStartsWith { value, prefix } => super::string_expr(value, cursor, graph, context)
            .map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value { mut cursor, value } => {
                    let result = graph.bool_instruction(
                        &mut cursor,
                        I::StringStartsWith {
                            value,
                            prefix: prefix.clone(),
                        },
                    );
                    DraftFlow::value(cursor, result)
                }
            }),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::Bool),
            |cursor, graph, context| bool_expr(true_, cursor, graph, context),
            |cursor, graph, context| bool_expr(false_, cursor, graph, context),
            DraftBool::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Bool),
            bool_expr,
            DraftBool::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Bool),
            bool_expr,
            DraftBool::from_ref,
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
            super::case_lowering(graph, context, StoredValueShape::Bool),
            bool_expr,
            DraftBool::from_ref,
        ),
        E::Block { .. }
        | E::And { .. }
        | E::Or { .. }
        | E::BitArrayMatches { .. }
        | E::CustomMatches { .. }
        | E::ListLengthEquals { .. }
        | E::ListLengthAtLeast { .. } => bool_paths(expression, cursor, graph, context)
            .map(|paths| materialize_paths(paths, graph)),
    }
}

fn compare_int(
    left: &module::IntExpr,
    right: &module::IntExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    kind: impl FnOnce(
        super::super::DraftInt,
        super::super::DraftInt,
    ) -> super::super::instruction::DraftBoolInstruction,
) -> Representability<DraftFlow<DraftBool>> {
    super::int_expr(left, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        DraftFlow::Value {
            cursor,
            value: left,
        } => super::int_expr(right, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: right,
            } => {
                let value = graph.bool_instruction(&mut cursor, kind(left, right));
                DraftFlow::value(cursor, value)
            }
        }),
    })
}

fn compare_float(
    left: &module::FloatExpr,
    right: &module::FloatExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    kind: impl FnOnce(
        super::super::DraftFloat,
        super::super::DraftFloat,
    ) -> super::super::instruction::DraftBoolInstruction,
) -> Representability<DraftFlow<DraftBool>> {
    super::float_expr(left, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        DraftFlow::Value {
            cursor,
            value: left,
        } => super::float_expr(right, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: right,
            } => {
                let value = graph.bool_instruction(&mut cursor, kind(left, right));
                DraftFlow::value(cursor, value)
            }
        }),
    })
}

fn compare_values(
    left: &module::Expr,
    right: &module::Expr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    kind: impl FnOnce(
        super::super::DraftValueRef,
        super::super::DraftValueRef,
    ) -> super::super::instruction::DraftBoolInstruction,
) -> Representability<DraftFlow<DraftBool>> {
    expr(left, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        DraftFlow::Value {
            cursor,
            value: left,
        } => expr(right, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: right,
            } => {
                let value = graph.bool_instruction(&mut cursor, kind(left, right));
                DraftFlow::value(cursor, value)
            }
        }),
    })
}

fn list_length_paths(
    expression: &module::ListExpr,
    length: usize,
    equals: bool,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<BoolPaths> {
    list::list_expr(expression, cursor, graph, context).map(|flow| match flow {
        DraftFlow::Diverged => BoolPaths::Diverged,
        DraftFlow::Value { mut cursor, value } => {
            let empty_only = matches!(
                value.shape(),
                StoredValueShape::List(item)
                    if matches!(item.as_ref(), SpecializedValueShape::Parameter(_))
            );
            if empty_only {
                let matched = length == 0;
                return if matched {
                    BoolPaths::True(cursor)
                } else {
                    BoolPaths::False(cursor)
                };
            }
            let kind = if equals {
                super::super::instruction::DraftBoolInstruction::ListLengthEquals {
                    value: value.clone(),
                    length,
                }
            } else {
                super::super::instruction::DraftBoolInstruction::ListLengthAtLeast {
                    value: value.clone(),
                    length,
                }
            };
            let result = graph.bool_instruction(&mut cursor, kind);
            let scope = cursor.scope().clone();
            let true_ = graph.empty_block(scope.clone());
            let false_ = graph.empty_block(scope);
            graph.finish_bool_branch(cursor, result, true_.id(), false_.id());
            BoolPaths::Both { true_, false_ }
        }
    })
}

fn merge_and(right: BoolPaths, false_: DraftCursor, graph: &mut DraftGraph) -> BoolPaths {
    match right {
        BoolPaths::Diverged => BoolPaths::False(false_),
        BoolPaths::True(cursor) => BoolPaths::Both {
            true_: cursor,
            false_,
        },
        BoolPaths::False(cursor) => BoolPaths::False(merge_cursors(false_, cursor, graph)),
        BoolPaths::Both {
            true_,
            false_: right_false,
        } => BoolPaths::Both {
            true_,
            false_: merge_cursors(false_, right_false, graph),
        },
    }
}

fn merge_or(true_: DraftCursor, right: BoolPaths, graph: &mut DraftGraph) -> BoolPaths {
    match right {
        BoolPaths::Diverged => BoolPaths::True(true_),
        BoolPaths::True(cursor) => BoolPaths::True(merge_cursors(true_, cursor, graph)),
        BoolPaths::False(cursor) => BoolPaths::Both {
            true_,
            false_: cursor,
        },
        BoolPaths::Both {
            true_: right_true,
            false_,
        } => BoolPaths::Both {
            true_: merge_cursors(true_, right_true, graph),
            false_,
        },
    }
}

fn merge_cursors(left: DraftCursor, right: DraftCursor, graph: &mut DraftGraph) -> DraftCursor {
    let scope = left.scope().clone();
    let merge = graph.empty_block(scope);
    let target = merge.id();
    graph.finish_jump(left, target, Vec::new());
    graph.finish_jump(right, target, Vec::new());
    merge
}

#[cfg(test)]
mod tests {
    use super::bool_expr;
    use crate::plan::execution::lowering::graph::draft::{DraftGraphBuilder, DraftNeverReturn};
    use crate::plan::execution::lowering::graph::{DraftCursor, DraftFlow, DraftValueRef};
    use crate::plan::execution::lowering::specialization::Representability;
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, GenericExpr, GenericLocal, GenericLocalId, IntExpr, ListExpr,
        PanicExpr, PanicSite, StringExpr, TypeParameterId, ValueType,
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

    fn static_path(paths: Representability<super::BoolPaths>) -> (bool, DraftCursor) {
        match paths {
            Representability::Inhabited(super::BoolPaths::True(cursor)) => (true, cursor),
            Representability::Inhabited(super::BoolPaths::False(cursor)) => (false, cursor),
            _ => panic!("fixture should produce a static Bool path"),
        }
    }

    #[test]
    fn bool_graph_lowering_preserves_direct_source_stops_and_short_circuit_shapes() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let dynamic = || {
            BoolExpr::equal(
                Expr::int(IntExpr::value(1.into())),
                Expr::int(IntExpr::value(1.into())),
            )
        };
        let expressions = vec![
            (
                BoolExpr::and(BoolExpr::panic(panic()), BoolExpr::value(true)),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::or(BoolExpr::panic(panic()), BoolExpr::value(false)),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::and(dynamic(), BoolExpr::panic(panic())),
                FlowOutcome::Value,
            ),
            (
                BoolExpr::and(dynamic(), BoolExpr::value(true)),
                FlowOutcome::Value,
            ),
            (
                BoolExpr::and(dynamic(), BoolExpr::value(false)),
                FlowOutcome::Value,
            ),
            (BoolExpr::and(dynamic(), dynamic()), FlowOutcome::Value),
            (
                BoolExpr::or(dynamic(), BoolExpr::panic(panic())),
                FlowOutcome::Value,
            ),
            (
                BoolExpr::or(dynamic(), BoolExpr::value(true)),
                FlowOutcome::Value,
            ),
            (
                BoolExpr::or(dynamic(), BoolExpr::value(false)),
                FlowOutcome::Value,
            ),
            (BoolExpr::or(dynamic(), dynamic()), FlowOutcome::Value),
            (
                BoolExpr::not(BoolExpr::panic(panic())),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::lt_int(IntExpr::panic(panic()), IntExpr::value(1.into())),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::lt_int(IntExpr::value(1.into()), IntExpr::panic(panic())),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::lt_float(FloatExpr::panic(panic()), FloatExpr::value(1.0)),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::lt_float(FloatExpr::value(1.0), FloatExpr::panic(panic())),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::equal(
                    Expr::int(IntExpr::panic(panic())),
                    Expr::int(IntExpr::value(1.into())),
                ),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::equal(
                    Expr::int(IntExpr::value(1.into())),
                    Expr::int(IntExpr::panic(panic())),
                ),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::string_starts_with(StringExpr::panic(panic()), "prefix".into()),
                FlowOutcome::Diverged,
            ),
            (
                BoolExpr::list_length_equals(ListExpr::panic(panic(), ValueType::Int), 0),
                FlowOutcome::Diverged,
            ),
        ];
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for (expression, expected) in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(bool_expr(&expression, cursor, &mut graph, &mut context,)),
                expected,
            );
        }

        let parameter = TypeParameterId(0);
        let generic = || {
            Expr::generic(GenericExpr::local_get(
                GenericLocal::new(GenericLocalId(0), parameter),
                "value".into(),
            ))
        };
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(bool_expr(
                &BoolExpr::equal(generic(), generic()),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );
    }

    #[test]
    fn empty_parameter_list_length_is_decided_without_a_length_instruction() {
        let parameter = TypeParameterId(0);
        let expression = ListExpr::value(Vec::new(), ValueType::Parameter(parameter));

        for (length, expected) in [(0, true), (1, false)] {
            let mut context =
                crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
            let (mut graph, cursor) =
                DraftGraphBuilder::<DraftNeverReturn, ()>::new(Vec::new(), Vec::new());
            let paths = super::list_length_paths(
                &expression,
                length,
                true,
                cursor,
                &mut graph,
                &mut context,
            );

            let (actual, cursor) = static_path(paths);
            assert_eq!(actual, expected);
            graph.finish_source_stop(
                cursor,
                crate::plan::execution::graph::SourceStopKind::Panic,
                None,
                PanicSite::unknown(),
            );
            let lowered = super::super::super::super::freeze::freeze(graph, &mut context);
            assert_eq!(lowered.body.block_graph().blocks().len(), 1);
            assert_eq!(
                lowered.body.block_graph().blocks()[0].instructions().len(),
                1
            );
        }
    }

    #[test]
    #[should_panic(expected = "fixture should produce a static Bool path")]
    fn static_path_value_rejects_a_non_static_fixture() {
        static_path(Representability::Uninhabited);
    }

    #[test]
    fn bool_operators_preserve_operand_evaluation_order() {
        for (expression, expected) in [
            ("!failed_bool(\"operand\")", "panic: operand"),
            (
                "failed_int(\"left\") < failed_int(\"right\")",
                "panic: left",
            ),
            ("1 < failed_int(\"right\")", "panic: right"),
            (
                "failed_float(\"left\") <. failed_float(\"right\")",
                "panic: left",
            ),
            ("1.0 <. failed_float(\"right\")", "panic: right"),
            (
                "failed_int(\"left\") == failed_int(\"right\")",
                "panic: left",
            ),
            ("1 == failed_int(\"right\")", "panic: right"),
            ("failed_callable()()", "panic: callable"),
        ] {
            assert_eq!(run(expression), expected);
        }
    }

    #[test]
    fn short_circuit_paths_preserve_source_stops() {
        for (expression, expected) in [
            (
                "failed_bool(\"left\") && failed_bool(\"right\")",
                "panic: left",
            ),
            (
                "failed_bool(\"left\") || failed_bool(\"right\")",
                "panic: left",
            ),
            ("and_then(True)", "panic: right"),
            ("or_else(False)", "panic: right"),
        ] {
            assert_eq!(run(expression), expected);
        }
    }

    fn run(expression: &str) -> String {
        let source = format!(
            r#"
fn failed_bool(message: String) -> Bool {{ panic as message }}
fn failed_int(message: String) -> Int {{ panic as message }}
fn failed_float(message: String) -> Float {{ panic as message }}
fn failed_callable() -> fn() -> Bool {{ panic as "callable" }}

fn and_then(value: Bool) -> Bool {{ value && failed_bool("right") }}
fn or_else(value: Bool) -> Bool {{ value || failed_bool("right") }}

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
