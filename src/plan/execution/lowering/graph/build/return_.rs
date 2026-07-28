use super::{
    DraftCursor, DraftFlow, DraftGraph, DraftGraphBuilder, DraftGraphValue, DraftNeverReturn,
};
use crate::plan::ValueShape;
use crate::plan::execution;
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::module;
use std::convert::Infallible;

type NeverCallTarget = crate::plan::FunctionCallTarget<execution::function::NeverFunctionId>;

pub(in crate::plan::execution::lowering::graph) fn build_function_graph<
    ModuleExpression,
    ModuleFunction,
    DraftReturn,
    TailCall,
>(
    template: &module::FunctionTemplate,
    body: &module::ReturnBody<ModuleExpression, ModuleFunction>,
    context: &mut super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
    lower_function: impl Copy
    + Fn(&ModuleFunction, &mut super::LoweringContext) -> Representability<TailCall>,
) -> Representability<DraftGraphBuilder<DraftReturn, TailCall>>
where
    DraftReturn: DraftGraphValue,
{
    let (mut graph, cursor) = graph_builder(template, context);
    lower_prefix(template.steps(), cursor, &mut graph, context)
        .and_then(|flow| {
            flow.and_then(|cursor, ()| {
                lower_return_body(
                    body,
                    cursor,
                    &mut graph,
                    context,
                    lower_expression,
                    lower_function,
                )
                .map(|()| DraftFlow::<()>::Diverged)
            })
        })
        .map(|_| graph)
}

pub(in crate::plan::execution::lowering::graph) fn build_never_function_graph<
    ModuleExpression,
    ModuleFunction,
>(
    template: &module::FunctionTemplate,
    body: &module::ReturnBody<ModuleExpression, ModuleFunction>,
    context: &mut super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::LoweringContext,
    ) -> Representability<()>,
    lower_function: impl Copy
    + Fn(
        &ModuleFunction,
        &mut super::LoweringContext,
    ) -> Representability<NeverCallTarget>,
) -> Representability<DraftGraphBuilder<DraftNeverReturn, NeverCallTarget>> {
    let (mut graph, cursor) = graph_builder(template, context);
    lower_prefix(template.steps(), cursor, &mut graph, context)
        .and_then(|flow| {
            flow.and_then(|cursor, ()| {
                lower_never_return_body(
                    body,
                    cursor,
                    &mut graph,
                    context,
                    lower_expression,
                    lower_function,
                )
                .map(|()| DraftFlow::<()>::Diverged)
            })
        })
        .map(|_| graph)
}

pub(in crate::plan::execution::lowering::graph) fn build_constant_graph<
    ModuleExpression,
    DraftReturn,
>(
    expression: &ModuleExpression,
    context: &mut super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
) -> Representability<DraftGraphBuilder<DraftReturn, Infallible>>
where
    DraftReturn: DraftGraphValue,
{
    let (mut graph, cursor) = DraftGraphBuilder::new(Vec::new(), Vec::new());
    lower_expression(expression, cursor, &mut graph, context).map(|flow| {
        flow.fold((), |cursor, value| {
            graph.finish_return(cursor, value);
        });
        graph
    })
}

fn graph_builder<Return, TailCall>(
    template: &module::FunctionTemplate,
    context: &super::LoweringContext,
) -> (DraftGraphBuilder<Return, TailCall>, DraftCursor) {
    let params = stored_params(template.entry().params(), context);
    let captures = stored_captures(template.entry().captures(), context);
    DraftGraphBuilder::new(params, captures)
}

fn stored_params(
    params: &[module::Param],
    context: &super::LoweringContext,
) -> Vec<(super::local::LocalKey, StoredValueShape)> {
    params
        .iter()
        .filter_map(|param| stored_entry_slot(param.local(), param.shape(), context))
        .collect()
}

fn stored_captures(
    captures: &[module::ParamSlot],
    context: &super::LoweringContext,
) -> Vec<(super::local::LocalKey, StoredValueShape)> {
    captures
        .iter()
        .filter_map(|capture| stored_entry_slot(capture.local(), capture.shape(), context))
        .collect()
}

fn stored_entry_slot(
    local: &module::ParamLocal,
    shape: &ValueShape,
    context: &super::LoweringContext,
) -> Option<(super::local::LocalKey, StoredValueShape)> {
    let shape = context.concrete_value_shape(shape);
    context
        .representations
        .stored_shape(&shape)
        .map(|shape| (super::local::param_local_key(local), shape))
}

fn lower_prefix(
    steps: &[module::Step],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::LoweringContext,
) -> Representability<DraftFlow<()>> {
    super::step::steps(steps, cursor, graph, context)
}

fn lower_return_body<ModuleExpression, ModuleFunction, DraftReturn, TailCall>(
    body: &module::ReturnBody<ModuleExpression, ModuleFunction>,
    cursor: DraftCursor,
    graph: &mut DraftGraphBuilder<DraftReturn, TailCall>,
    context: &mut super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
    lower_function: impl Copy
    + Fn(&ModuleFunction, &mut super::LoweringContext) -> Representability<TailCall>,
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

struct ReturnSwitch<'a, Pattern, ModuleExpression, ModuleFunction> {
    clauses: &'a [(
        Pattern,
        module::ReturnBody<ModuleExpression, ModuleFunction>,
    )],
    fallback: &'a module::ReturnBody<ModuleExpression, ModuleFunction>,
}

fn lower_switch<Pattern, Subject, ModuleExpression, ModuleFunction, DraftReturn, TailCall>(
    subject: Representability<DraftFlow<Subject>>,
    switch: ReturnSwitch<'_, Pattern, ModuleExpression, ModuleFunction>,
    graph: &mut DraftGraphBuilder<DraftReturn, TailCall>,
    context: &mut super::LoweringContext,
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
        &mut super::LoweringContext,
    ) -> Representability<DraftFlow<DraftReturn>>,
    lower_function: impl Copy
    + Fn(&ModuleFunction, &mut super::LoweringContext) -> Representability<TailCall>,
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
            branch_cursors
                .into_iter()
                .enumerate()
                .fold(
                    Representability::Inhabited(()),
                    |lowered, (index, cursor)| {
                        let branch = &clauses[index].1;
                        lowered.and_then(|()| {
                            lower_return_body(
                                branch,
                                cursor,
                                graph,
                                context,
                                lower_expression,
                                lower_function,
                            )
                        })
                    },
                )
                .and_then(|()| {
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
    })
}

fn lower_never_return_body<ModuleExpression, ModuleFunction>(
    body: &module::ReturnBody<ModuleExpression, ModuleFunction>,
    cursor: DraftCursor,
    graph: &mut DraftGraphBuilder<DraftNeverReturn, NeverCallTarget>,
    context: &mut super::LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        DraftCursor,
        &mut DraftGraph,
        &mut super::LoweringContext,
    ) -> Representability<()>,
    lower_function: impl Copy
    + Fn(
        &ModuleFunction,
        &mut super::LoweringContext,
    ) -> Representability<NeverCallTarget>,
) -> Representability<()> {
    use module::ReturnBodyKind as B;

    match body.kind() {
        B::Expr(expression) => lower_expression(expression, cursor, graph, context),
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
                    super::expression::bool::BoolPaths::True(cursor) => lower_never_return_body(
                        true_body,
                        cursor,
                        graph,
                        context,
                        lower_expression,
                        lower_function,
                    ),
                    super::expression::bool::BoolPaths::False(cursor) => lower_never_return_body(
                        false_body,
                        cursor,
                        graph,
                        context,
                        lower_expression,
                        lower_function,
                    ),
                    super::expression::bool::BoolPaths::Both { true_, false_ } => {
                        lower_never_return_body(
                            true_body,
                            true_,
                            graph,
                            context,
                            lower_expression,
                            lower_function,
                        )
                        .and_then(|()| {
                            lower_never_return_body(
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
        } => lower_never_switch(
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
        } => lower_never_switch(
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
        } => lower_never_switch(
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
                    lower_never_return_body(
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

fn lower_never_switch<Pattern, Subject, ModuleExpression, ModuleFunction>(
    subject: Representability<DraftFlow<Subject>>,
    switch: ReturnSwitch<'_, Pattern, ModuleExpression, ModuleFunction>,
    graph: &mut DraftGraphBuilder<DraftNeverReturn, NeverCallTarget>,
    context: &mut super::LoweringContext,
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
        &mut super::LoweringContext,
    ) -> Representability<()>,
    lower_function: impl Copy
    + Fn(
        &ModuleFunction,
        &mut super::LoweringContext,
    ) -> Representability<NeverCallTarget>,
) -> Representability<()>
where
    Pattern: Clone,
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
            branch_cursors
                .into_iter()
                .enumerate()
                .fold(
                    Representability::Inhabited(()),
                    |lowered, (index, cursor)| {
                        let branch = &clauses[index].1;
                        lowered.and_then(|()| {
                            lower_never_return_body(
                                branch,
                                cursor,
                                graph,
                                context,
                                lower_expression,
                                lower_function,
                            )
                        })
                    },
                )
                .and_then(|()| {
                    lower_never_return_body(
                        fallback,
                        fallback_cursor,
                        graph,
                        context,
                        lower_expression,
                        lower_function,
                    )
                })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::super::instruction::DraftIntInstruction;
    use super::super::{
        DraftCursor, DraftFlow, DraftGraph, DraftGraphBuilder, DraftInt, DraftNeverReturn,
    };
    use crate::plan::execution::function::IntFunctionId as ExecutionIntFunctionId;
    use crate::plan::execution::graph::ParamLocal;
    use crate::plan::execution::graph::{BlockId, SourceStopKind, Terminator};
    use crate::plan::execution::lowering::specialization::Representability;
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionInstantiation, FunctionTemplate, FunctionTemplateId,
        GenericExpr, GenericLocal, GenericLocalId, IntExpr, IntFunctionId, IntLocalId, PanicExpr,
        PanicSite, Param, ParamLocal as ModuleParamLocal, ParamSlot, ReturnBody, ReturnExpr, Step,
        StringExpr, TupleExpr, TypeParameterId, ValueShape, ValueType,
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

    fn finish_never_tuple_expression(
        _expression: &TupleExpr,
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

    fn finish_never_function(
        function: &FunctionInstantiation,
        context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<super::NeverCallTarget> {
        context.never_function_id(function).map(|function| {
            crate::plan::FunctionCallTarget::new(function, crate::plan::HostCallSite::unknown())
        })
    }

    fn finish_never_call_target(
        target: &crate::plan::FunctionCallTarget<FunctionInstantiation>,
        context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<super::NeverCallTarget> {
        context
            .never_function_id(target.function())
            .map(|function| crate::plan::FunctionCallTarget::new(function, target.site().clone()))
    }

    #[derive(Clone, Copy)]
    enum ExpectedSwitch {
        Int,
        Float,
        String,
    }

    fn bool_branch_targets(terminator: &Terminator) -> (BlockId, BlockId) {
        match terminator {
            Terminator::BoolBranch(branch) => (branch.true_().target(), branch.false_().target()),
            _ => panic!("fixture should contain a Bool branch"),
        }
    }

    fn switch_targets(expected: ExpectedSwitch, terminator: &Terminator) -> (BlockId, BlockId) {
        match (expected, terminator) {
            (ExpectedSwitch::Int, Terminator::IntSwitch(switch)) => {
                assert_eq!(switch.clauses()[0].0, 1.into());
                (switch.clauses()[0].1.target(), switch.fallback().target())
            }
            (ExpectedSwitch::Float, Terminator::FloatSwitch(switch)) => {
                assert_eq!(switch.clauses()[0].0, 1.0);
                (switch.clauses()[0].1.target(), switch.fallback().target())
            }
            (ExpectedSwitch::String, Terminator::StringSwitch(switch)) => {
                assert_eq!(switch.clauses()[0].0, "one");
                (switch.clauses()[0].1.target(), switch.fallback().target())
            }
            _ => panic!("unexpected switch terminator"),
        }
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
        let lowered = super::super::super::freeze::freeze(graph, &mut context);

        assert_eq!(lowered.parameter_count, 1);
        assert_eq!(lowered.body.block_graph().blocks().len(), 1);
        assert_eq!(
            lowered
                .body
                .block_graph()
                .block(lowered.body.block_graph().entry())
                .params()
                .len(),
            2,
        );
        assert_eq!(
            lowered
                .body
                .block_graph()
                .block(lowered.body.block_graph().entry())
                .params()[0]
                .local(),
            &ParamLocal::Int(crate::plan::execution::graph::IntLocalId(0)),
        );
        assert_eq!(
            lowered
                .body
                .block_graph()
                .block(lowered.body.block_graph().entry())
                .params()[1]
                .local(),
            &ParamLocal::Int(crate::plan::execution::graph::IntLocalId(1)),
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
    fn never_return_tail_calls_preserve_plain_and_source_call_targets() {
        let function = crate::plan::monomorphic_function_instantiation(
            0,
            crate::plan::FunctionShape::new(Vec::new(), ValueShape::Int),
        );
        let plain_body =
            ReturnBody::<IntExpr, FunctionInstantiation>::tail_call(function.clone(), Vec::new());
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );

        assert_eq!(
            super::lower_never_return_body(
                &plain_body,
                cursor,
                &mut graph,
                &mut context,
                reject_never_expression,
                finish_never_function,
            ),
            Representability::Inhabited(()),
        );
        let lowered = super::super::super::freeze::freeze(graph, &mut context);
        assert_eq!(lowered.body.block_graph().blocks().len(), 1);

        let parameter = TypeParameterId(0);
        let target = crate::plan::FunctionCallTarget::new(
            function,
            crate::plan::HostCallSite::new(
                "main".into(),
                "tail".into(),
                crate::plan::SourceSpan::new(4, 8),
            ),
        );
        let source_body = ReturnBody::<
            GenericExpr,
            crate::plan::FunctionCallTarget<FunctionInstantiation>,
        >::tail_call(target, Vec::new());
        let template = FunctionTemplate::new(
            FunctionTemplateId::new(3),
            "tail".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::generic_body(parameter, source_body.clone()),
        );
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let lowered = super::super::super::lower_never_function_graph(
            &template,
            &source_body,
            &mut context,
            super::super::expression::generic::never_expr,
            finish_never_call_target,
        );

        assert_eq!(lowered.map(|_| ()), Representability::Inhabited(()));
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
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );
        assert_eq!(
            super::lower_never_return_body(
                &true_body,
                cursor,
                &mut graph,
                &mut context,
                finish_never_expression,
                finish_never_function,
            ),
            Representability::Inhabited(()),
        );
        let lowered = super::super::super::freeze::freeze(graph, &mut context);
        assert_eq!(lowered.body.block_graph().blocks().len(), 1);
        let terminator = lowered
            .body
            .block_graph()
            .block(lowered.body.block_graph().entry())
            .terminator();
        assert_eq!(source_stop_kind(terminator), SourceStopKind::Panic);

        let false_body = ReturnBody::<IntExpr, FunctionInstantiation>::bool_case(
            BoolExpr::value(false),
            ReturnBody::expr(IntExpr::value(1.into())),
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );
        assert_eq!(
            super::lower_never_return_body(
                &false_body,
                cursor,
                &mut graph,
                &mut context,
                finish_never_expression,
                finish_never_function,
            ),
            Representability::Inhabited(()),
        );
        let lowered = super::super::super::freeze::freeze(graph, &mut context);
        assert_eq!(lowered.body.block_graph().blocks().len(), 1);
        assert_eq!(
            source_stop_kind(
                lowered
                    .body
                    .block_graph()
                    .block(lowered.body.block_graph().entry())
                    .terminator()
            ),
            SourceStopKind::Panic,
        );

        let dynamic_body = ReturnBody::<IntExpr, FunctionInstantiation>::bool_case(
            BoolExpr::equal(
                Expr::int(IntExpr::value(1.into())),
                Expr::int(IntExpr::value(1.into())),
            ),
            ReturnBody::expr(IntExpr::value(1.into())),
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );
        assert_eq!(
            super::lower_never_return_body(
                &dynamic_body,
                cursor,
                &mut graph,
                &mut context,
                finish_never_expression,
                finish_never_function,
            ),
            Representability::Inhabited(()),
        );
        let lowered = super::super::super::freeze::freeze(graph, &mut context);
        assert_eq!(lowered.body.block_graph().blocks().len(), 3);
        let (true_, false_) = bool_branch_targets(
            lowered
                .body
                .block_graph()
                .block(lowered.body.block_graph().entry())
                .terminator(),
        );
        assert_eq!(true_, BlockId::new(1));
        assert_eq!(false_, BlockId::new(2));
        assert_eq!(
            source_stop_kind(lowered.body.block_graph().block(true_).terminator()),
            SourceStopKind::Panic,
        );
        assert_eq!(
            source_stop_kind(lowered.body.block_graph().block(false_).terminator()),
            SourceStopKind::Panic,
        );

        let panic_subject = ReturnBody::<IntExpr, FunctionInstantiation>::bool_case(
            BoolExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
            ReturnBody::expr(IntExpr::value(1.into())),
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );
        assert_eq!(
            super::lower_never_return_body(
                &panic_subject,
                cursor,
                &mut graph,
                &mut context,
                finish_never_expression,
                finish_never_function,
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
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );

        assert_eq!(
            super::lower_never_return_body(
                &body,
                cursor,
                &mut graph,
                &mut context,
                reject_never_expression,
                finish_never_function,
            ),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn never_return_switches_lower_every_branch_and_fallback() {
        let bodies = [
            (
                ReturnBody::<IntExpr, FunctionInstantiation>::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), ReturnBody::expr(IntExpr::value(1.into())))],
                    ReturnBody::expr(IntExpr::value(2.into())),
                ),
                ExpectedSwitch::Int,
            ),
            (
                ReturnBody::<IntExpr, FunctionInstantiation>::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(IntExpr::value(1.into())))],
                    ReturnBody::expr(IntExpr::value(2.into())),
                ),
                ExpectedSwitch::Float,
            ),
            (
                ReturnBody::<IntExpr, FunctionInstantiation>::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), ReturnBody::expr(IntExpr::value(1.into())))],
                    ReturnBody::expr(IntExpr::value(2.into())),
                ),
                ExpectedSwitch::String,
            ),
        ];

        for (body, expected) in bodies {
            let mut context =
                crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
            let (mut graph, cursor) =
                DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                    Vec::new(),
                    Vec::new(),
                );
            assert_eq!(
                super::lower_never_return_body(
                    &body,
                    cursor,
                    &mut graph,
                    &mut context,
                    finish_never_expression,
                    finish_never_function,
                ),
                Representability::Inhabited(()),
            );
            let lowered = super::super::super::freeze::freeze(graph, &mut context);
            assert_eq!(lowered.body.block_graph().blocks().len(), 3);

            let (branch, fallback) = switch_targets(
                expected,
                lowered
                    .body
                    .block_graph()
                    .block(lowered.body.block_graph().entry())
                    .terminator(),
            );
            assert_eq!(branch, BlockId::new(1));
            assert_eq!(fallback, BlockId::new(2));
            assert_eq!(
                source_stop_kind(lowered.body.block_graph().block(branch).terminator()),
                SourceStopKind::Panic,
            );
            assert_eq!(
                source_stop_kind(lowered.body.block_graph().block(fallback).terminator()),
                SourceStopKind::Panic,
            );
        }
    }

    #[test]
    fn never_return_blocks_and_function_prefixes_preserve_source_stops() {
        let block = ReturnBody::<IntExpr, FunctionInstantiation>::block(
            vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
            ReturnBody::expr(IntExpr::value(2.into())),
        );
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );
        assert_eq!(
            super::lower_never_return_body(
                &block,
                cursor,
                &mut graph,
                &mut context,
                finish_never_expression,
                finish_never_function,
            ),
            Representability::Inhabited(()),
        );
        let lowered = super::super::super::freeze::freeze(graph, &mut context);
        let entry = lowered
            .body
            .block_graph()
            .block(lowered.body.block_graph().entry());
        assert_eq!(entry.instructions().len(), 1);
        assert_eq!(source_stop_kind(entry.terminator()), SourceStopKind::Panic);

        let parameter = TypeParameterId(0);
        let body = ReturnBody::expr(GenericExpr::panic(
            parameter,
            PanicExpr::panic_at(None, PanicSite::unknown()),
        ));
        let template = FunctionTemplate::new(
            FunctionTemplateId::new(3),
            "prefix".into(),
            Vec::new(),
            vec![Step::evaluate(Expr::int(IntExpr::panic(
                PanicExpr::panic_at(None, PanicSite::unknown()),
            )))],
            ReturnExpr::generic_body(parameter, body.clone()),
        );
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let lowered = super::super::super::lower_never_function_graph(
            &template,
            &body,
            &mut context,
            super::super::expression::generic::never_expr,
            finish_never_call_target,
        );
        assert_eq!(
            lowered.map(|lowered| {
                let entry = lowered
                    .body
                    .block_graph()
                    .block(lowered.body.block_graph().entry());
                (
                    entry.instructions().len(),
                    source_stop_kind(entry.terminator()),
                )
            }),
            Representability::Inhabited((0, SourceStopKind::Panic)),
        );

        let tuple_body = ReturnBody::<TupleExpr, FunctionInstantiation>::block(
            vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
            ReturnBody::expr(TupleExpr::value(
                vec![Expr::int(IntExpr::value(2.into()))],
                vec![ValueType::Int],
            )),
        );
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, super::NeverCallTarget>::new(
                Vec::new(),
                Vec::new(),
            );
        assert_eq!(
            super::lower_never_return_body(
                &tuple_body,
                cursor,
                &mut graph,
                &mut context,
                finish_never_tuple_expression,
                finish_never_function,
            ),
            Representability::Inhabited(()),
        );
        let lowered = super::super::super::freeze::freeze(graph, &mut context);
        let entry = lowered
            .body
            .block_graph()
            .block(lowered.body.block_graph().entry());
        assert_eq!(entry.instructions().len(), 1);
        assert_eq!(source_stop_kind(entry.terminator()), SourceStopKind::Panic);
    }

    #[test]
    #[should_panic(expected = "fixture should contain a Bool branch")]
    fn bool_branch_targets_rejects_the_wrong_fixture_shape() {
        bool_branch_targets(&Terminator::Exit(
            crate::plan::execution::graph::BlockGraphExitId::new(0),
        ));
    }

    #[test]
    #[should_panic(expected = "unexpected switch terminator")]
    fn switch_targets_rejects_the_wrong_fixture_shape() {
        switch_targets(
            ExpectedSwitch::Int,
            &Terminator::Exit(crate::plan::execution::graph::BlockGraphExitId::new(0)),
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain a source stop")]
    fn source_stop_kind_rejects_the_wrong_fixture_shape() {
        source_stop_kind(&Terminator::Exit(
            crate::plan::execution::graph::BlockGraphExitId::new(0),
        ));
    }

    fn source_stop_kind(terminator: &Terminator) -> SourceStopKind {
        match terminator {
            Terminator::SourceStop(stop) => stop.kind(),
            _ => panic!("fixture should contain a source stop"),
        }
    }
}
