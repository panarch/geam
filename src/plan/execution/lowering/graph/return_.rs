use super::freeze::FreezeGraphValue;
use super::{
    DraftCursor, DraftFlow, DraftGraph, DraftGraphBuilder, DraftGraphValue, DraftNeverReturn,
};
use crate::plan::execution;
use crate::plan::execution::lowering::specialization::{Representability, ValueRepresentation};
use crate::plan::module;

pub(in crate::plan::execution::lowering) fn lower_function_graph<
    ModuleExpression,
    DraftReturn,
    FrozenReturn,
    TailCall,
>(
    template: &module::FunctionTemplate,
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut super::super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut super::super::LoweringContext,
    ) -> Representability<TailCall>,
) -> Representability<
    super::LoweredFunctionGraph<execution::graph::FunctionGraph<FrozenReturn, TailCall>>,
>
where
    DraftReturn: DraftGraphValue + FreezeGraphValue<Frozen = FrozenReturn>,
    TailCall: Clone,
{
    let (mut graph, cursor) = graph_builder(template, context);
    lower_prefix(template.steps(), cursor, &mut graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(super::freeze::freeze(graph, context)),
        DraftFlow::Value { cursor, value: () } => lower_return_body(
            body,
            cursor,
            &mut graph,
            context,
            lower_expression,
            lower_function,
        )
        .map(|()| super::freeze::freeze(graph, context)),
    })
}

pub(in crate::plan::execution::lowering) fn lower_never_function_graph<ModuleExpression>(
    template: &module::FunctionTemplate,
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut super::super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<()>,
) -> Representability<
    super::LoweredFunctionGraph<
        execution::graph::FunctionGraph<execution::graph::NeverReturn, execution::NeverFunctionId>,
    >,
> {
    let (mut graph, cursor) = graph_builder(template, context);
    lower_prefix(template.steps(), cursor, &mut graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(super::freeze::freeze(graph, context)),
        DraftFlow::Value { cursor, value: () } => {
            lower_never_return_body(body, cursor, &mut graph, context, lower_expression)
                .map(|()| super::freeze::freeze(graph, context))
        }
    })
}

pub(in crate::plan::execution::lowering) fn lower_constant_graph<
    ModuleExpression,
    DraftReturn,
    FrozenReturn,
>(
    expression: &ModuleExpression,
    context: &mut super::super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
) -> Representability<execution::ConstantProgram<FrozenReturn>>
where
    DraftReturn: DraftGraphValue + FreezeGraphValue<Frozen = FrozenReturn>,
{
    let (mut graph, cursor) = DraftGraphBuilder::new(Vec::new(), Vec::new());
    lower_expression(expression, cursor, &mut graph, context).map(|flow| {
        flow.fold((), |cursor, value| {
            graph.finish_return(cursor, value);
        });
        execution::ConstantProgram::new(super::freeze::freeze(graph, context).body)
    })
}

fn graph_builder<Return, TailCall>(
    template: &module::FunctionTemplate,
    context: &super::super::LoweringContext,
) -> (DraftGraphBuilder<Return, TailCall>, DraftCursor) {
    let params = template
        .entry()
        .params()
        .iter()
        .map(|param| (param.local(), param.shape()))
        .filter_map(|(local, shape)| {
            let shape = context.concrete_value_shape(shape);
            match context.representations.representation(&shape) {
                ValueRepresentation::Uninhabited(_) => None,
                ValueRepresentation::Stored(shape) => {
                    Some((super::super::local::param_local_key(local), shape))
                }
            }
        })
        .collect();
    let captures = template
        .entry()
        .captures()
        .iter()
        .map(|capture| (capture.local(), capture.shape()))
        .filter_map(|(local, shape)| {
            let shape = context.concrete_value_shape(shape);
            match context.representations.representation(&shape) {
                ValueRepresentation::Uninhabited(_) => None,
                ValueRepresentation::Stored(shape) => {
                    Some((super::super::local::param_local_key(local), shape))
                }
            }
        })
        .collect();
    DraftGraphBuilder::new(params, captures)
}

fn lower_prefix(
    steps: &[module::Step],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftFlow<()>> {
    super::step::steps(steps, cursor, graph, context)
}

fn lower_return_body<ModuleExpression, DraftReturn, TailCall>(
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    cursor: DraftCursor,
    graph: &mut DraftGraphBuilder<DraftReturn, TailCall>,
    context: &mut super::super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut super::super::LoweringContext,
    ) -> Representability<TailCall>,
) -> Representability<()>
where
    DraftReturn: DraftGraphValue,
{
    use module::ReturnBodyKind as B;

    match body.kind() {
        B::Expr(expression) => lower_expression(expression, cursor, graph, context).map(|flow| {
            flow.fold((), |cursor, value| {
                graph.finish_return(cursor, value);
            });
        }),
        B::TailCall { function, args } => {
            super::expression::call_args(args, cursor, graph, context).and_then(|flow| {
                flow.fold(Representability::Inhabited(()), |cursor, args| {
                    lower_function(function, context).map(|function| {
                        graph.finish_tail_call(cursor, function, args);
                    })
                })
            })
        }
        B::BoolCase {
            subject,
            true_: true_body,
            false_: false_body,
        } => {
            super::expression::bool::bool_paths(subject, cursor, graph, context).and_then(|paths| {
                match paths {
                    super::expression::bool::BoolPaths::Diverged => Representability::Inhabited(()),
                    super::expression::bool::BoolPaths::True(cursor) => lower_return_body(
                        true_body,
                        cursor,
                        graph,
                        context,
                        lower_expression,
                        lower_function,
                    ),
                    super::expression::bool::BoolPaths::False(cursor) => lower_return_body(
                        false_body,
                        cursor,
                        graph,
                        context,
                        lower_expression,
                        lower_function,
                    ),
                    super::expression::bool::BoolPaths::Both { true_, false_ } => {
                        lower_return_body(
                            true_body,
                            true_,
                            graph,
                            context,
                            lower_expression,
                            lower_function,
                        )
                        .and_then(|()| {
                            lower_return_body(
                                false_body,
                                false_,
                                graph,
                                context,
                                lower_expression,
                                lower_function,
                            )
                        })
                    }
                }
            })
        }
        B::IntCase {
            subject,
            clauses,
            fallback,
        } => lower_switch(
            super::expression::int_expr(subject, cursor, graph, context),
            ReturnSwitch { clauses, fallback },
            graph,
            context,
            |cursor, subject, clauses, fallback, graph| {
                graph.finish_int_switch(cursor, subject, clauses, fallback);
            },
            lower_expression,
            lower_function,
        ),
        B::FloatCase {
            subject,
            clauses,
            fallback,
        } => lower_switch(
            super::expression::float_expr(subject, cursor, graph, context),
            ReturnSwitch { clauses, fallback },
            graph,
            context,
            |cursor, subject, clauses, fallback, graph| {
                graph.finish_float_switch(cursor, subject, clauses, fallback);
            },
            lower_expression,
            lower_function,
        ),
        B::StringCase {
            subject,
            clauses,
            fallback,
        } => lower_switch(
            super::expression::string_expr(subject, cursor, graph, context),
            ReturnSwitch { clauses, fallback },
            graph,
            context,
            |cursor, subject, clauses, fallback, graph| {
                graph.finish_string_switch(cursor, subject, clauses, fallback);
            },
            lower_expression,
            lower_function,
        ),
        B::Block { steps, return_ } => {
            super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                flow.fold(Representability::Inhabited(()), |cursor, ()| {
                    lower_return_body(
                        return_,
                        cursor,
                        graph,
                        context,
                        lower_expression,
                        lower_function,
                    )
                })
            })
        }
    }
}

struct ReturnSwitch<'a, Pattern, ModuleExpression> {
    clauses: &'a [(
        Pattern,
        module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    )],
    fallback: &'a module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
}

fn lower_switch<Pattern, Subject, ModuleExpression, DraftReturn, TailCall>(
    subject: Representability<DraftFlow<Subject>>,
    switch: ReturnSwitch<'_, Pattern, ModuleExpression>,
    graph: &mut DraftGraphBuilder<DraftReturn, TailCall>,
    context: &mut super::super::LoweringContext,
    finish: impl FnOnce(
        DraftCursor,
        Subject,
        Vec<(Pattern, super::DraftBlockId)>,
        super::DraftBlockId,
        &mut DraftGraph,
    ),
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut super::super::LoweringContext,
    ) -> Representability<TailCall>,
) -> Representability<()>
where
    Pattern: Clone,
    DraftReturn: DraftGraphValue,
{
    let ReturnSwitch { clauses, fallback } = switch;
    subject.and_then(|flow| {
        flow.fold(Representability::Inhabited(()), |cursor, subject| {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            finish(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
                graph,
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower_return_body(
                    branch,
                    cursor,
                    graph,
                    context,
                    lower_expression,
                    lower_function,
                ) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            lower_return_body(
                fallback,
                fallback_cursor,
                graph,
                context,
                lower_expression,
                lower_function,
            )
        })
    })
}

fn lower_never_return_body<ModuleExpression>(
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    cursor: DraftCursor,
    graph: &mut DraftGraphBuilder<DraftNeverReturn, execution::NeverFunctionId>,
    context: &mut super::super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<()>,
) -> Representability<()> {
    use module::ReturnBodyKind as B;

    match body.kind() {
        B::Expr(expression) => lower_expression(expression, cursor, graph, context),
        B::TailCall { function, args } => {
            super::expression::call_args(args, cursor, graph, context).and_then(|flow| {
                flow.fold(Representability::Inhabited(()), |cursor, args| {
                    context.never_function_id(function).map(|function| {
                        graph.finish_tail_call(cursor, function, args);
                    })
                })
            })
        }
        B::BoolCase {
            subject,
            true_: true_body,
            false_: false_body,
        } => {
            super::expression::bool::bool_paths(subject, cursor, graph, context).and_then(|paths| {
                match paths {
                    super::expression::bool::BoolPaths::Diverged => Representability::Inhabited(()),
                    super::expression::bool::BoolPaths::True(cursor) => {
                        lower_never_return_body(true_body, cursor, graph, context, lower_expression)
                    }
                    super::expression::bool::BoolPaths::False(cursor) => lower_never_return_body(
                        false_body,
                        cursor,
                        graph,
                        context,
                        lower_expression,
                    ),
                    super::expression::bool::BoolPaths::Both { true_, false_ } => {
                        lower_never_return_body(true_body, true_, graph, context, lower_expression)
                            .and_then(|()| {
                                lower_never_return_body(
                                    false_body,
                                    false_,
                                    graph,
                                    context,
                                    lower_expression,
                                )
                            })
                    }
                }
            })
        }
        B::IntCase {
            subject,
            clauses,
            fallback,
        } => lower_never_switch(
            super::expression::int_expr(subject, cursor, graph, context),
            clauses,
            fallback,
            graph,
            context,
            |cursor, subject, clauses, fallback, graph| {
                graph.finish_int_switch(cursor, subject, clauses, fallback);
            },
            lower_expression,
        ),
        B::FloatCase {
            subject,
            clauses,
            fallback,
        } => lower_never_switch(
            super::expression::float_expr(subject, cursor, graph, context),
            clauses,
            fallback,
            graph,
            context,
            |cursor, subject, clauses, fallback, graph| {
                graph.finish_float_switch(cursor, subject, clauses, fallback);
            },
            lower_expression,
        ),
        B::StringCase {
            subject,
            clauses,
            fallback,
        } => lower_never_switch(
            super::expression::string_expr(subject, cursor, graph, context),
            clauses,
            fallback,
            graph,
            context,
            |cursor, subject, clauses, fallback, graph| {
                graph.finish_string_switch(cursor, subject, clauses, fallback);
            },
            lower_expression,
        ),
        B::Block { steps, return_ } => {
            super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                flow.fold(Representability::Inhabited(()), |cursor, ()| {
                    lower_never_return_body(return_, cursor, graph, context, lower_expression)
                })
            })
        }
    }
}

fn lower_never_switch<Pattern, Subject, ModuleExpression>(
    subject: Representability<DraftFlow<Subject>>,
    clauses: &[(
        Pattern,
        module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    )],
    fallback: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    graph: &mut DraftGraphBuilder<DraftNeverReturn, execution::NeverFunctionId>,
    context: &mut super::super::LoweringContext,
    finish: impl FnOnce(
        DraftCursor,
        Subject,
        Vec<(Pattern, super::DraftBlockId)>,
        super::DraftBlockId,
        &mut DraftGraph,
    ),
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<()>,
) -> Representability<()>
where
    Pattern: Clone,
{
    subject.and_then(|flow| {
        flow.fold(Representability::Inhabited(()), |cursor, subject| {
            let scope = cursor.scope().clone();
            let branch_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope);
            finish(
                cursor,
                subject,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), branch_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
                graph,
            );
            for (index, cursor) in branch_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower_never_return_body(branch, cursor, graph, context, lower_expression) {
                    Representability::Inhabited(()) => {}
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            lower_never_return_body(fallback, fallback_cursor, graph, context, lower_expression)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::super::instruction::DraftIntInstruction;
    use super::super::{
        DraftCursor, DraftFlow, DraftGraph, DraftGraphBuilder, DraftInt, DraftNeverReturn,
    };
    use crate::plan::execution::graph::{SourceStopKind, Terminator};
    use crate::plan::execution::lowering::specialization::Representability;
    use crate::plan::execution::{IntFunctionId as ExecutionIntFunctionId, ParamLocal};
    use crate::plan::{
        BoolExpr, FunctionInstantiation, FunctionTemplate, FunctionTemplateId, GenericLocal,
        GenericLocalId, IntExpr, IntFunctionId, IntLocalId, PanicExpr, PanicSite, Param,
        ParamLocal as ModuleParamLocal, ParamSlot, ReturnBody, ReturnExpr, TypeParameterId,
        ValueShape,
    };

    fn reject_int_expression(
        _expression: &IntExpr,
        _cursor: DraftCursor,
        _graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<DraftFlow<DraftInt>> {
        Representability::Uninhabited
    }

    fn reject_int_function(
        _function: &FunctionInstantiation,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<ExecutionIntFunctionId> {
        Representability::Uninhabited
    }

    fn finish_never_expression(
        _expression: &IntExpr,
        cursor: DraftCursor,
        graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<()> {
        graph.finish_source_stop(cursor, SourceStopKind::Panic, None, PanicSite::unknown());
        Representability::Inhabited(())
    }

    fn reject_never_expression(
        _expression: &IntExpr,
        _cursor: DraftCursor,
        _graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<()> {
        Representability::Uninhabited
    }

    #[test]
    fn graph_entry_omits_uninhabited_parameters_and_captures() {
        let parameter = TypeParameterId(0);
        let template = FunctionTemplate::with_captures(
            FunctionTemplateId::new(2),
            "entry".into(),
            vec![
                Param::named_shape(
                    ModuleParamLocal::generic(GenericLocal::new(GenericLocalId(0), parameter)),
                    "uninhabited".into(),
                    ValueShape::Parameter(parameter),
                ),
                Param::named_shape(
                    ModuleParamLocal::int(IntLocalId(0)),
                    "stored".into(),
                    ValueShape::Int,
                ),
            ],
            vec![
                ParamSlot::new(
                    ModuleParamLocal::generic(GenericLocal::new(GenericLocalId(1), parameter)),
                    ValueShape::Parameter(parameter),
                ),
                ParamSlot::new(ModuleParamLocal::int(IntLocalId(1)), ValueShape::Int),
            ],
            Vec::new(),
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
        );
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, mut cursor) =
            super::graph_builder::<DraftInt, ExecutionIntFunctionId>(&template, &context);
        let value = graph.int_instruction(&mut cursor, DraftIntInstruction::Value(1.into()));
        graph.finish_return(cursor, value);
        let lowered = super::super::freeze::freeze(graph, &mut context);

        assert_eq!(lowered.parameter_count, 1);
        assert_eq!(lowered.body.blocks().len(), 1);
        assert_eq!(lowered.body.block(lowered.body.entry()).params().len(), 2,);
        assert_eq!(
            lowered.body.block(lowered.body.entry()).params()[0].local(),
            &ParamLocal::Int(crate::plan::execution::IntLocalId(0)),
        );
        assert_eq!(
            lowered.body.block(lowered.body.entry()).params()[1].local(),
            &ParamLocal::Int(crate::plan::execution::IntLocalId(1)),
        );
    }

    #[test]
    fn return_body_propagates_divergence_and_uninhabited_switch_branches() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let panic_subject = ReturnBody::<IntExpr, FunctionInstantiation>::bool_case(
            BoolExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ReturnBody::expr(IntExpr::value(1.into())),
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftInt, ExecutionIntFunctionId>::new(Vec::new(), Vec::new());
        assert_eq!(
            super::lower_return_body(
                &panic_subject,
                cursor,
                &mut graph,
                &mut context,
                super::super::expression::int_expr,
                reject_int_function,
            ),
            Representability::Inhabited(()),
        );

        let switch = ReturnBody::<IntExpr, FunctionInstantiation>::int_case(
            IntExpr::value(1.into()),
            vec![(1.into(), ReturnBody::expr(IntExpr::value(1.into())))],
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftInt, ExecutionIntFunctionId>::new(Vec::new(), Vec::new());
        assert_eq!(
            super::lower_return_body(
                &switch,
                cursor,
                &mut graph,
                &mut context,
                reject_int_expression,
                reject_int_function,
            ),
            Representability::Uninhabited,
        );

        let tail_call = ReturnBody::<IntExpr, FunctionInstantiation>::tail_call(
            crate::plan::monomorphic_function_instantiation(
                0,
                crate::plan::FunctionShape::new(Vec::new(), ValueShape::Int),
            ),
            Vec::new(),
        );
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftInt, ExecutionIntFunctionId>::new(Vec::new(), Vec::new());
        assert_eq!(
            super::lower_return_body(
                &tail_call,
                cursor,
                &mut graph,
                &mut context,
                reject_int_expression,
                reject_int_function,
            ),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn never_return_body_preserves_static_and_diverging_bool_paths() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let true_body = ReturnBody::<IntExpr, FunctionInstantiation>::bool_case(
            BoolExpr::value(true),
            ReturnBody::expr(IntExpr::value(1.into())),
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) = DraftGraphBuilder::<
            DraftNeverReturn,
            crate::plan::execution::NeverFunctionId,
        >::new(Vec::new(), Vec::new());
        assert_eq!(
            super::lower_never_return_body(
                &true_body,
                cursor,
                &mut graph,
                &mut context,
                finish_never_expression,
            ),
            Representability::Inhabited(()),
        );
        let lowered = super::super::freeze::freeze(graph, &mut context);
        assert_eq!(lowered.body.blocks().len(), 1);
        let terminator = lowered.body.block(lowered.body.entry()).terminator();
        assert_eq!(source_stop_kind(terminator), SourceStopKind::Panic);

        let panic_subject = ReturnBody::<IntExpr, FunctionInstantiation>::bool_case(
            BoolExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ReturnBody::expr(IntExpr::value(1.into())),
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) = DraftGraphBuilder::<
            DraftNeverReturn,
            crate::plan::execution::NeverFunctionId,
        >::new(Vec::new(), Vec::new());
        assert_eq!(
            super::lower_never_return_body(
                &panic_subject,
                cursor,
                &mut graph,
                &mut context,
                finish_never_expression,
            ),
            Representability::Inhabited(()),
        );
    }

    #[test]
    fn never_return_switch_propagates_an_uninhabited_branch() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let body = ReturnBody::<IntExpr, FunctionInstantiation>::int_case(
            IntExpr::value(1.into()),
            vec![(1.into(), ReturnBody::expr(IntExpr::value(1.into())))],
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) = DraftGraphBuilder::<
            DraftNeverReturn,
            crate::plan::execution::NeverFunctionId,
        >::new(Vec::new(), Vec::new());

        assert_eq!(
            super::lower_never_return_body(
                &body,
                cursor,
                &mut graph,
                &mut context,
                reject_never_expression,
            ),
            Representability::Uninhabited,
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain a source stop")]
    fn source_stop_kind_rejects_the_wrong_fixture_shape() {
        source_stop_kind(&Terminator::<(), ()>::Return(()));
    }

    fn source_stop_kind<Return, TailCall>(
        terminator: &Terminator<Return, TailCall>,
    ) -> SourceStopKind {
        match terminator {
            Terminator::SourceStop { kind, .. } => *kind,
            _ => panic!("fixture should contain a source stop"),
        }
    }
}
