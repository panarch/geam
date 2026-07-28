use super::super::{call_args, custom, list, tuple};
use super::{closure, function_function_expr, reference, source_stop};
use crate::plan::execution::graph::FunctionTarget;
use crate::plan::execution::lowering::graph::{
    DraftCursor, DraftFlow, DraftGraph, DraftNeverFunction,
};
use crate::plan::execution::lowering::specialization::{
    Representability, SpecializedFunctionShape, UninhabitedValueShape,
};
use crate::plan::{execution, module};

pub(super) fn function_expr(
    expression: &module::FunctionExpr,
    _proof: &UninhabitedValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<crate::plan::execution::lowering::graph::DraftFunction>> {
    let shape = context.concrete_function_shape(expression.shape());
    let lowered = match expression.kind() {
        module::FunctionExprKind::Generic(expression) => {
            generic_function_expr(expression, &shape, cursor, graph, context)
        }
        module::FunctionExprKind::Tuple(expression) => {
            tuple_function_kind(expression.kind(), &shape, cursor, graph, context)
        }
        module::FunctionExprKind::Custom(expression) => {
            custom_function_kind(expression.kind(), &shape, cursor, graph, context)
        }
        _ => return Representability::Uninhabited,
    };
    lowered.map(|flow| flow.map(|value| value.value().clone()))
}

pub(in crate::plan::execution::lowering) fn generic_function_expr(
    expression: &module::GenericFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftNeverFunction>> {
    fn lower(
        kind: &module::GenericFunctionExprKind,
        shape: &SpecializedFunctionShape,
        cursor: DraftCursor,
        graph: &mut DraftGraph,
        context: &mut super::super::super::LoweringContext,
    ) -> Representability<DraftFlow<DraftNeverFunction>> {
        super::generic::lower_executable_kind(
            kind,
            shape,
            cursor,
            graph,
            context,
            super::generic::executable_kind_lowering(
                DraftNeverFunction::new,
                |value, context| {
                    context
                        .generic_never_function_constant(value, shape)
                        .map(|id| id.index())
                },
                |function, context| {
                    context
                        .never_function_id(function)
                        .map(FunctionTarget::Never)
                },
                |function, context| {
                    let type_ = context.generic_function_type(shape);
                    context
                        .never_function_function_id(function, type_)
                        .map(execution::function::FunctionFunctionId::Never)
                },
                |branch, cursor, graph, context| lower(branch, shape, cursor, graph, context),
            ),
        )
    }

    lower(expression.kind(), shape, cursor, graph, context)
}

macro_rules! fixed_never_function {
    ($name:ident, $module_kind:ident, $constant:ident, $local_kind:ident) => {
        fn $name(
            kind: &module::$module_kind,
            shape: &SpecializedFunctionShape,
            cursor: DraftCursor,
            graph: &mut DraftGraph,
            context: &mut super::super::super::LoweringContext,
        ) -> Representability<DraftFlow<DraftNeverFunction>> {
            use super::super::super::instruction::DraftFunctionInstruction as I;
            use module::$module_kind as E;

            let stored =
                crate::plan::execution::lowering::specialization::StoredValueShape::Function(
                    Box::new(shape.clone()),
                );
            match kind {
                E::Constant(value) => context.$constant(value).map(|id| {
                    let mut cursor = cursor;
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Constant(execution::constant::ConstantId::new(id.index())),
                    );
                    DraftFlow::value(cursor, DraftNeverFunction::new(value))
                }),
                E::Reference(value) => {
                    context
                        .never_function_id(value.instantiation())
                        .map(|target| {
                            reference(shape.clone(), FunctionTarget::Never(target), cursor, graph)
                                .map(DraftNeverFunction::new)
                        })
                }
                E::Closure {
                    function, captures, ..
                } => context.never_function_id(function).and_then(|target| {
                    closure(
                        function,
                        captures,
                        shape.clone(),
                        FunctionTarget::Never(target),
                        cursor,
                        graph,
                        context,
                    )
                    .map(|flow| flow.map(DraftNeverFunction::new))
                }),
                E::LocalGet { local, name: _ } => {
                    let value = cursor
                        .scope()
                        .function(super::super::super::local::LocalKey::new(
                            super::super::super::local::LocalKind::$local_kind,
                            local.0,
                        ));
                    Representability::Inhabited(DraftFlow::value(
                        cursor,
                        DraftNeverFunction::new(value),
                    ))
                }
                E::Call {
                    function,
                    args,
                    site,
                    ..
                } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value {
                        mut cursor,
                        value: args,
                    } => {
                        let type_ = context.generic_function_type(shape);
                        context
                            .never_function_function_id(function, type_)
                            .map(|function| {
                                let function =
                                    execution::function::FunctionFunctionId::Never(function);
                                let value = graph.function_instruction(
                                    &mut cursor,
                                    shape.clone(),
                                    I::Call {
                                        function,
                                        args,
                                        site: site.clone(),
                                    },
                                );
                                DraftFlow::value(cursor, DraftNeverFunction::new(value))
                            })
                    }
                }),
                E::FunctionCall {
                    function,
                    args,
                    site,
                    ..
                } => function_function_expr(function, cursor, graph, context).and_then(|flow| {
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
                                let value = graph.function_instruction(
                                    &mut cursor,
                                    shape.clone(),
                                    I::FunctionCall {
                                        function: function.value().clone(),
                                        args,
                                        site: site.clone(),
                                    },
                                );
                                DraftFlow::value(cursor, DraftNeverFunction::new(value))
                            }
                        }),
                    }
                }),
                E::TupleIndex {
                    tuple: source,
                    index,
                    ..
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
                        DraftFlow::value(cursor, DraftNeverFunction::new(value))
                    }
                }),
                E::CustomField(access) => {
                    custom::custom_expr(access.source(), cursor, graph, context).map(|flow| {
                        match flow {
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
                                DraftFlow::value(cursor, DraftNeverFunction::new(value))
                            }
                        }
                    })
                }
                E::ListIndex {
                    list: source,
                    index,
                    ..
                } => list::function_list_expr(source, cursor, graph, context).map(
                    |flow| match flow {
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
                            DraftFlow::value(cursor, DraftNeverFunction::new(value))
                        }
                    },
                ),
                E::Panic(value) => source_stop(value, cursor, graph, context)
                    .map(|flow| flow.map(DraftNeverFunction::new)),
                E::BoolCase {
                    subject,
                    true_,
                    false_,
                } => super::super::bool_case(
                    subject,
                    cursor,
                    super::super::case_lowering(graph, context, stored),
                    |cursor, graph, context| $name(true_.kind(), shape, cursor, graph, context),
                    |cursor, graph, context| $name(false_.kind(), shape, cursor, graph, context),
                    DraftNeverFunction::from_ref,
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
                    |branch, cursor, graph, context| {
                        $name(branch.kind(), shape, cursor, graph, context)
                    },
                    DraftNeverFunction::from_ref,
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
                    |branch, cursor, graph, context| {
                        $name(branch.kind(), shape, cursor, graph, context)
                    },
                    DraftNeverFunction::from_ref,
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
                    |branch, cursor, graph, context| {
                        $name(branch.kind(), shape, cursor, graph, context)
                    },
                    DraftNeverFunction::from_ref,
                ),
                E::Block { steps, return_ } => super::super::super::step::steps(
                    steps, cursor, graph, context,
                )
                .and_then(|flow| match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        $name(return_.kind(), shape, cursor, graph, context)
                    }
                }),
            }
        }
    };
}

fixed_never_function!(
    tuple_function_kind,
    TupleFunctionExprKind,
    tuple_never_function_constant,
    TupleFunction
);

pub(in crate::plan::execution::lowering) fn tuple_function_expr(
    expression: &module::TupleFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftNeverFunction>> {
    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.type_().clone(),
    ));
    tuple_function_kind(expression.kind(), &shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn custom_function_kind(
    kind: &module::CustomFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftNeverFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::CustomFunctionExprKind as E;

    let stored = crate::plan::execution::lowering::specialization::StoredValueShape::Function(
        Box::new(shape.clone()),
    );
    match kind {
        E::Constant(value) => context.custom_never_function_constant(value).map(|id| {
            let mut cursor = cursor;
            let value = graph.function_instruction(
                &mut cursor,
                shape.clone(),
                I::Constant(execution::constant::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, DraftNeverFunction::new(value))
        }),
        E::Constructor(_) => Representability::Uninhabited,
        E::Reference(value) => context
            .never_function_id(value.instantiation())
            .map(|target| {
                reference(shape.clone(), FunctionTarget::Never(target), cursor, graph)
                    .map(DraftNeverFunction::new)
            }),
        E::Closure { function, captures } => {
            context.never_function_id(function).and_then(|target| {
                closure(
                    function,
                    captures,
                    shape.clone(),
                    FunctionTarget::Never(target),
                    cursor,
                    graph,
                    context,
                )
                .map(|flow| flow.map(DraftNeverFunction::new))
            })
        }
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::CustomFunction,
                    local.id().0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, DraftNeverFunction::new(value)))
        }
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => {
                let type_ = context.generic_function_type(shape);
                context
                    .never_function_function_id(function, type_)
                    .map(|function| {
                        let function = execution::function::FunctionFunctionId::Never(function);
                        let value = graph.function_instruction(
                            &mut cursor,
                            shape.clone(),
                            I::Call {
                                function,
                                args,
                                site: site.clone(),
                            },
                        );
                        DraftFlow::value(cursor, DraftNeverFunction::new(value))
                    })
            }
        }),
        E::FunctionCall {
            function,
            args,
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
                    DraftFlow::value(cursor, DraftNeverFunction::new(value))
                }
            }),
        }),
        E::TupleIndex {
            tuple: source,
            index,
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
                DraftFlow::value(cursor, DraftNeverFunction::new(value))
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
                    DraftFlow::value(cursor, DraftNeverFunction::new(value))
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
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
                DraftFlow::value(cursor, DraftNeverFunction::new(value))
            }
        }),
        E::Panic(value) => {
            source_stop(value, cursor, graph, context).map(|flow| flow.map(DraftNeverFunction::new))
        }
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| custom_function_kind(true_, shape, cursor, graph, context),
            |cursor, graph, context| custom_function_kind(false_, shape, cursor, graph, context),
            DraftNeverFunction::from_ref,
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
            |branch, cursor, graph, context| {
                custom_function_kind(branch, shape, cursor, graph, context)
            },
            DraftNeverFunction::from_ref,
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
            |branch, cursor, graph, context| {
                custom_function_kind(branch, shape, cursor, graph, context)
            },
            DraftNeverFunction::from_ref,
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
            |branch, cursor, graph, context| {
                custom_function_kind(branch, shape, cursor, graph, context)
            },
            DraftNeverFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        custom_function_kind(return_, shape, cursor, graph, context)
                    }
                }
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn custom_function_expr(
    expression: &module::CustomFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftNeverFunction>> {
    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.custom_function_type().to_function_type(),
    ));
    custom_function_kind(expression.kind(), &shape, cursor, graph, context)
}

#[cfg(test)]
mod tests {
    use super::{custom_function_kind, function_expr, tuple_function_kind};
    use crate::plan::execution::lowering::graph::draft::DraftGraphBuilder;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{
        Representability, StoredValueShape, UninhabitedValueShape,
    };
    use crate::plan::{
        CallArg, CustomConstructor, CustomConstructorRefinement, CustomFieldAccess,
        CustomFunctionExpr, CustomFunctionType, CustomType, CustomTypeDefinition, CustomTypeName,
        CustomTypeParameterId, CustomTypePublicity, CustomTypeTemplate, CustomValueShape, Expr,
        FunctionExpr, FunctionFunctionExpr, FunctionFunctionType, FunctionShape, FunctionType,
        GenericExpr, IntFunctionExpr, ListExpr, PanicExpr, PanicSite, Step, TupleExpr,
        TupleFunctionExpr, TupleFunctionLocalId, TypeParameterId, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };

    #[test]
    fn recursive_never_callable_handoffs_execute_every_owner_path() {
        let source = include_str!(
            "../../../../../../../../tests/fixtures/execution/functions/generic_recursive_never_function_handoffs.gleam"
        );
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("recursive never callable fixture should compile");
        let module =
            crate::plan_module(typed).expect("recursive never callable fixture should plan");
        let execution = crate::ExecutionPlan::from_module_plan(module);

        assert_eq!(
            crate::run_main(&execution, &mut Vec::new()),
            Ok(crate::Value::Tuple(vec![crate::Value::Bool(true); 62])),
        );
    }

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

    fn source_stop() -> PanicExpr {
        PanicExpr::panic_at(None, PanicSite::unknown())
    }

    fn diverging_argument(parameter: TypeParameterId) -> CallArg {
        CallArg::new(Expr::generic(GenericExpr::panic(parameter, source_stop())))
    }

    fn callable_provider(
        parameter: TypeParameterId,
        callable: FunctionShape,
    ) -> crate::plan::FunctionInstantiation {
        monomorphic_function_instantiation(
            99,
            FunctionShape::new(
                vec![ValueShape::Parameter(parameter)],
                ValueShape::Function(Box::new(callable)),
            ),
        )
    }

    fn holder_definition(name: CustomTypeName) -> CustomTypeDefinition {
        CustomTypeDefinition::new(
            name,
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![crate::plan::CustomConstructorDefinition::new(
                "Holder".into(),
                0,
                vec![crate::plan::CustomFieldDefinition::new(
                    Some("value".into()),
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        )
    }

    fn empty_definition(name: CustomTypeName) -> CustomTypeDefinition {
        CustomTypeDefinition::new(
            name,
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            Vec::new(),
        )
    }

    fn custom_source(holder: &CustomTypeName, callable: &FunctionShape) -> CustomFieldAccess {
        let shape = CustomValueShape::new(
            holder.clone(),
            vec![ValueShape::Function(Box::new(callable.clone()))],
            CustomConstructorRefinement::Exact(0),
        );
        CustomFieldAccess::new(
            crate::plan::CustomExpr::panic_shape(source_stop(), shape),
            0,
            None,
        )
    }

    fn function_list(callable: FunctionType) -> crate::plan::FunctionListExpr {
        ListExpr::panic(source_stop(), ValueType::Function(Box::new(callable)))
            .into_function()
            .expect("a function item type should create a function list")
    }

    #[test]
    fn never_function_owner_rejects_other_callable_families() {
        let parameter = TypeParameterId(0);
        let type_ = FunctionType::new(Vec::new(), ValueType::Int);
        let expression = FunctionExpr::int_with_shape(
            IntFunctionExpr::panic(source_stop(), type_),
            FunctionShape::new(Vec::new(), ValueShape::Parameter(parameter)),
        );
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        assert_eq!(
            flow_outcome(function_expr(
                &expression,
                &UninhabitedValueShape::Parameter(parameter),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );
    }

    #[test]
    fn never_function_owner_preserves_tuple_locals_and_source_stops() {
        let parameter = TypeParameterId(0);
        let callable_shape = FunctionShape::new(
            Vec::new(),
            ValueShape::Tuple(vec![ValueShape::Parameter(parameter)].into()),
        );
        let callable_type = callable_shape.type_();
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let shape = context.concrete_function_shape(&callable_shape);
        let local = TupleFunctionLocalId(0);
        let key = crate::plan::execution::lowering::local::LocalKey::new(
            crate::plan::execution::lowering::local::LocalKind::TupleFunction,
            local.0,
        );
        let stored = StoredValueShape::Function(Box::new(shape.clone()));

        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(vec![(key, stored.clone())], Vec::new());
        let local_expression =
            TupleFunctionExpr::local_get(local, "callable".into(), callable_type.clone());
        assert_eq!(
            flow_outcome(tuple_function_kind(
                local_expression.kind(),
                &shape,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(vec![(key, stored)], Vec::new());
        let local_expression =
            FunctionExpr::tuple_with_shape(local_expression, callable_shape.clone());
        assert_eq!(
            flow_outcome(function_expr(
                &local_expression,
                &UninhabitedValueShape::Parameter(parameter),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let source_stop = FunctionExpr::tuple_with_shape(
            TupleFunctionExpr::panic(source_stop(), callable_type),
            callable_shape,
        );
        assert_eq!(
            flow_outcome(function_expr(
                &source_stop,
                &UninhabitedValueShape::Parameter(parameter),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Diverged,
        );
    }

    #[test]
    fn tuple_never_callable_stops_at_each_diverging_owner_source() {
        let parameter = TypeParameterId(0);
        let holder = CustomTypeName::new("geam".into(), "main".into(), "Holder".into());
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            holder_definition(holder.clone()),
        ]);
        let return_shape = ValueShape::Tuple(vec![ValueShape::Parameter(parameter)].into());
        let callable_shape = FunctionShape::new(Vec::new(), return_shape.clone());
        let callable_type = callable_shape.type_();
        let shape = context.concrete_function_shape(&callable_shape);
        let provider = callable_provider(parameter, callable_shape.clone());
        let return_ = || TupleFunctionExpr::panic(source_stop(), callable_type.clone());
        let expressions = [
            TupleFunctionExpr::call(
                provider,
                vec![diverging_argument(parameter)],
                callable_type.clone(),
            ),
            TupleFunctionExpr::function_call(
                FunctionFunctionExpr::panic(
                    source_stop(),
                    FunctionFunctionType::from_shapes(Vec::new(), callable_shape.clone()),
                ),
                Vec::new(),
                callable_type.clone(),
            ),
            TupleFunctionExpr::tuple_index(
                TupleExpr::panic(
                    source_stop(),
                    vec![ValueType::Function(Box::new(callable_type.clone()))],
                ),
                0,
                callable_type.clone(),
            ),
            TupleFunctionExpr::custom_field(
                custom_source(&holder, &callable_shape),
                callable_type.clone(),
            ),
            TupleFunctionExpr::list_index(
                function_list(callable_type.clone()),
                0,
                callable_type.clone(),
            ),
            TupleFunctionExpr::block(
                vec![Step::evaluate(Expr::generic(GenericExpr::panic(
                    parameter,
                    source_stop(),
                )))],
                return_(),
            ),
        ];
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(tuple_function_kind(
                    expression.kind(),
                    &shape,
                    cursor,
                    &mut graph,
                    &mut context,
                )),
                FlowOutcome::Diverged,
            );
        }
    }

    #[test]
    fn custom_never_callable_stops_at_each_diverging_owner_source() {
        let parameter = TypeParameterId(0);
        let holder = CustomTypeName::new("geam".into(), "main".into(), "Holder".into());
        let empty = CustomTypeName::new("geam".into(), "main".into(), "Empty".into());
        let marker = CustomTypeName::new("geam".into(), "main".into(), "Marker".into());
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            holder_definition(holder.clone()),
            empty_definition(empty.clone()),
        ]);
        let return_shape =
            CustomValueShape::new(empty, Vec::new(), CustomConstructorRefinement::Any);
        let callable_type = CustomFunctionType::from_shapes(Vec::new(), return_shape.clone());
        let callable_shape = FunctionShape::new(Vec::new(), ValueShape::Custom(return_shape));
        let shape = context.concrete_function_shape(&callable_shape);
        let provider = callable_provider(parameter, callable_shape.clone());
        let return_ = || CustomFunctionExpr::panic(source_stop(), callable_type.clone());
        let expressions = [
            CustomFunctionExpr::call(
                provider,
                vec![diverging_argument(parameter)],
                callable_type.clone(),
            ),
            CustomFunctionExpr::try_function_call(
                FunctionFunctionExpr::panic(
                    source_stop(),
                    FunctionFunctionType::from_shapes(Vec::new(), callable_shape.clone()),
                ),
                Vec::new(),
            )
            .expect("the callable argument pack is exact"),
            CustomFunctionExpr::tuple_index(
                TupleExpr::panic(
                    source_stop(),
                    vec![ValueType::Function(Box::new(
                        callable_type.to_function_type(),
                    ))],
                ),
                0,
                callable_type.clone(),
            ),
            CustomFunctionExpr::custom_field(
                custom_source(&holder, &callable_shape),
                callable_type.clone(),
            ),
            CustomFunctionExpr::list_index(
                function_list(callable_type.to_function_type()),
                0,
                callable_type.clone(),
            ),
            CustomFunctionExpr::block(
                vec![Step::evaluate(Expr::generic(GenericExpr::panic(
                    parameter,
                    source_stop(),
                )))],
                return_(),
            ),
        ];
        let constructor = CustomFunctionExpr::constructor(CustomConstructor::new(
            CustomType::new(marker, Vec::new()),
            "Marker".into(),
            0,
            Vec::new(),
        ));
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(custom_function_kind(
                constructor.kind(),
                &shape,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );

        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(custom_function_kind(
                    expression.kind(),
                    &shape,
                    cursor,
                    &mut graph,
                    &mut context,
                )),
                FlowOutcome::Diverged,
            );
        }
    }
}
